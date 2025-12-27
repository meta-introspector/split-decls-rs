use anyhow::Result;
use syn::{File, Item, ItemFn, Expr, Stmt, Block};
use std::collections::HashMap;
use std::path::PathBuf;

/// RDF-backed interpreter that executes extracted split-decls-rs main routine step by step
pub struct RdfSynInterpreter {
    /// Loaded function declarations from split-decls-rs
    functions: HashMap<String, ItemFn>,
    /// RDF triples representing execution state
    rdf_state: Vec<(String, String, String)>,
    /// Current execution context
    execution_stack: Vec<ExecutionFrame>,
    /// Step counter for debugging
    step_count: usize,
}

#[derive(Debug, Clone)]
pub struct ExecutionFrame {
    function_name: String,
    variables: HashMap<String, SynValue>,
    current_statement: usize,
}

#[derive(Debug, Clone)]
pub enum SynValue {
    Path(PathBuf),
    String(String),
    Bool(bool),
    Unit,
}

impl RdfSynInterpreter {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            rdf_state: Vec::new(),
            execution_stack: Vec::new(),
            step_count: 0,
        }
    }

    /// Load the main function and its dependencies from extracted declarations
    pub fn load_main_routine(&mut self, decls_path: &std::path::Path) -> Result<()> {
        // Load all function declarations
        for entry in std::fs::read_dir(decls_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "rs") {
                let content = std::fs::read_to_string(&path)?;
                if let Ok(file) = syn::parse_str::<File>(&content) {
                    for item in file.items {
                        if let Item::Fn(func) = item {
                            let func_name = func.sig.ident.to_string();
                            self.functions.insert(func_name.clone(), func);
                            
                            // Add RDF triple for loaded function
                            self.add_rdf_triple(
                                format!("func:{}", func_name),
                                "rdf:type".to_string(),
                                "split_decls:Function".to_string()
                            );
                        }
                    }
                }
            }
        }
        
        println!("🧠 Loaded {} functions for RDF interpretation", self.functions.len());
        Ok(())
    }

    /// Start interpreting from the main function (or bootstrap equivalent)
    pub fn interpret_main_step_by_step(&mut self, entry_point: &str) -> Result<()> {
        println!("🚀 Starting RDF-backed interpretation of: {}", entry_point);
        
        // Initialize execution state in RDF
        self.add_rdf_triple(
            "exec:session".to_string(),
            "split_decls:entryPoint".to_string(),
            format!("func:{}", entry_point)
        );
        
        // Find the entry function
        if let Some(main_func) = self.functions.get(entry_point).cloned() {
            let frame = ExecutionFrame {
                function_name: entry_point.to_string(),
                variables: HashMap::new(),
                current_statement: 0,
            };
            
            self.execution_stack.push(frame);
            self.execute_function_stepwise(&main_func)?;
        } else {
            return Err(anyhow::anyhow!("Entry point '{}' not found", entry_point));
        }
        
        Ok(())
    }

    fn execute_function_stepwise(&mut self, func: &ItemFn) -> Result<()> {
        let func_name = func.sig.ident.to_string();
        println!("📋 Stepping through function: {}", func_name);
        
        // Record function entry in RDF
        self.step_count += 1;
        self.add_rdf_triple(
            format!("step:{}", self.step_count),
            "split_decls:executesFunction".to_string(),
            format!("func:{}", func_name)
        );
        
        // Execute each statement step by step
        for (i, stmt) in func.block.stmts.iter().enumerate() {
            self.step_count += 1;
            
            println!("  🔍 Step {}: Executing statement {}", self.step_count, i);
            
            // Record step in RDF
            self.add_rdf_triple(
                format!("step:{}", self.step_count),
                "split_decls:statementIndex".to_string(),
                i.to_string()
            );
            
            // Execute the statement
            let result = self.execute_statement_with_rdf(stmt)?;
            
            // Record result in RDF
            self.add_rdf_triple(
                format!("step:{}", self.step_count),
                "split_decls:result".to_string(),
                format!("{:?}", result)
            );
            
            // Interactive stepping
            println!("    [Press Enter to continue, 'q' to quit, 'rdf' to show state]");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            match input.trim() {
                "q" => break,
                "rdf" => self.show_rdf_state(),
                _ => {}
            }
        }
        
        Ok(())
    }

    fn execute_statement_with_rdf(&mut self, stmt: &Stmt) -> Result<SynValue> {
        match stmt {
            Stmt::Local(local) => {
                if let syn::Pat::Ident(ident) = &local.pat {
                    let var_name = ident.ident.to_string();
                    
                    let value = if let Some(init) = &local.init {
                        self.execute_expression_with_rdf(&init.expr)?
                    } else {
                        SynValue::Unit
                    };
                    
                    // Record variable binding in RDF
                    self.add_rdf_triple(
                        format!("var:{}", var_name),
                        "split_decls:boundInStep".to_string(),
                        format!("step:{}", self.step_count)
                    );
                    
                    // Update current frame
                    if let Some(frame) = self.execution_stack.last_mut() {
                        frame.variables.insert(var_name, value.clone());
                    }
                    
                    println!("    📝 Bound variable: {} = {:?}", ident.ident, value);
                    Ok(value)
                } else {
                    Ok(SynValue::Unit)
                }
            }
            Stmt::Expr(expr, _) => {
                println!("    ⚡ Evaluating expression");
                self.execute_expression_with_rdf(expr)
            }
            _ => Ok(SynValue::Unit),
        }
    }

    fn execute_expression_with_rdf(&mut self, expr: &Expr) -> Result<SynValue> {
        match expr {
            Expr::Call(call) => {
                if let Expr::Path(path) = &*call.func {
                    let func_name = path.path.segments.last().unwrap().ident.to_string();
                    
                    // Record function call in RDF
                    self.add_rdf_triple(
                        format!("step:{}", self.step_count),
                        "split_decls:calls".to_string(),
                        format!("func:{}", func_name)
                    );
                    
                    println!("      🔧 Calling: {}", func_name);
                    
                    // Handle built-in functions or recursive calls
                    match func_name.as_str() {
                        "println" => {
                            println!("      📢 Console output");
                            Ok(SynValue::Unit)
                        }
                        _ => {
                            // Try to call extracted function
                            if self.functions.contains_key(&func_name) {
                                println!("      🔄 Recursive call to: {}", func_name);
                                // For now, simulate the call
                                Ok(SynValue::Unit)
                            } else {
                                println!("      🎭 Simulating: {}", func_name);
                                Ok(SynValue::Unit)
                            }
                        }
                    }
                } else {
                    Ok(SynValue::Unit)
                }
            }
            Expr::Lit(lit) => {
                match &lit.lit {
                    syn::Lit::Str(s) => Ok(SynValue::String(s.value())),
                    syn::Lit::Bool(b) => Ok(SynValue::Bool(b.value)),
                    _ => Ok(SynValue::Unit),
                }
            }
            _ => Ok(SynValue::Unit),
        }
    }

    fn add_rdf_triple(&mut self, subject: String, predicate: String, object: String) {
        self.rdf_state.push((subject, predicate, object));
    }

    fn show_rdf_state(&self) {
        println!("📊 Current RDF State ({} triples):", self.rdf_state.len());
        for (i, (s, p, o)) in self.rdf_state.iter().enumerate().rev().take(10) {
            println!("  {} {} {} .", s, p, o);
        }
        if self.rdf_state.len() > 10 {
            println!("  ... and {} more triples", self.rdf_state.len() - 10);
        }
    }

    pub fn export_rdf_turtle(&self) -> String {
        let mut turtle = String::new();
        turtle.push_str("@prefix split_decls: <http://split-decls-rs.org/> .\n");
        turtle.push_str("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n\n");
        
        for (s, p, o) in &self.rdf_state {
            turtle.push_str(&format!("{} {} {} .\n", s, p, o));
        }
        
        turtle
    }
}
