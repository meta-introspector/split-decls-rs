use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;
use syn::{File, Expr, visit::Visit};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use quote::ToTokens;

#[derive(Debug, Clone)]
struct SubexpressionReport {
    pattern: String,
    count: usize,
    emoji_hash: String,
    locations: Vec<String>,
}

struct ExpressionVisitor {
    expressions: HashMap<String, Vec<String>>,
    current_file: String,
}

impl ExpressionVisitor {
    fn new(file_path: String) -> Self {
        Self {
            expressions: HashMap::new(),
            current_file: file_path,
        }
    }

    fn normalize_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(binary) => {
                format!("({} {} {})", 
                    self.normalize_expr(&binary.left),
                    quote::quote!(#binary.op).to_string(),
                    self.normalize_expr(&binary.right)
                )
            },
            Expr::Call(call) => {
                let args: Vec<String> = call.args.iter()
                    .map(|arg| self.normalize_expr(arg))
                    .collect();
                format!("call({})", args.join(","))
            },
            Expr::MethodCall(method) => {
                let args: Vec<String> = method.args.iter()
                    .map(|arg| self.normalize_expr(arg))
                    .collect();
                format!("{}.{}({})", 
                    self.normalize_expr(&method.receiver),
                    method.method,
                    args.join(",")
                )
            },
            Expr::Field(field) => {
                format!("{}.{}", 
                    self.normalize_expr(&field.base),
                    field.member.to_token_stream().to_string()
                )
            },
            Expr::Path(path) => {
                path.path.segments.iter()
                    .map(|seg| seg.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::")
            },
            Expr::Lit(lit) => format!("lit({})", quote::quote!(#lit).to_string()),
            _ => "expr".to_string(),
        }
    }
}

impl<'ast> Visit<'ast> for ExpressionVisitor {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        let normalized = self.normalize_expr(expr);
        
        // Only track expressions with some complexity
        if normalized.len() > 5 && normalized.contains('(') {
            self.expressions.entry(normalized)
                .or_insert_with(Vec::new)
                .push(self.current_file.clone());
        }
        
        syn::visit::visit_expr(self, expr);
    }
}

fn generate_emoji_hash(text: &str) -> String {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    let hash = hasher.finish();
    
    // Map hash to emoji combinations
    let emojis = [
        "🔥", "⚡", "🎯", "🚀", "💎", "🌟", "🔮", "🎨", 
        "🎪", "🎭", "🎨", "🎯", "🔥", "💫", "✨", "🌈",
        "🦄", "🐉", "🦋", "🌸", "🍀", "🎲", "🎪", "🎨"
    ];
    
    let primary = emojis[(hash % emojis.len() as u64) as usize];
    let secondary = emojis[((hash >> 8) % emojis.len() as u64) as usize];
    let tertiary = emojis[((hash >> 16) % emojis.len() as u64) as usize];
    
    format!("{}{}{}", primary, secondary, tertiary)
}

fn analyze_expressions_in_file(file_path: &Path) -> Result<HashMap<String, Vec<String>>> {
    let content = fs::read_to_string(file_path)?;
    let syntax_tree: File = syn::parse_file(&content)?;
    
    let mut visitor = ExpressionVisitor::new(file_path.to_string_lossy().to_string());
    
    for item in &syntax_tree.items {
        syn::visit::visit_item(&mut visitor, item);
    }
    
    Ok(visitor.expressions)
}

fn find_rust_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut rust_files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && !path.file_name().unwrap_or_default().to_string_lossy().starts_with('.') {
                rust_files.extend(find_rust_files(&path));
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                rust_files.push(path);
            }
        }
    }
    
    rust_files
}

fn main() -> Result<()> {
    println!("🔍 SUBEXPRESSION ANALYZER - Finding Common Patterns");
    println!("==================================================");
    
    let search_paths = vec!["src"];
    let mut all_expressions: HashMap<String, Vec<String>> = HashMap::new();
    
    for search_path in search_paths {
        let path = Path::new(search_path);
        if path.exists() {
            println!("\n📂 Analyzing: {}", search_path);
            let rust_files = find_rust_files(path);
            println!("   Found {} Rust files", rust_files.len());
            
            for file_path in rust_files.iter().take(50) { // Limit for performance
                if let Ok(expressions) = analyze_expressions_in_file(file_path) {
                    for (pattern, locations) in expressions {
                        all_expressions.entry(pattern)
                            .or_insert_with(Vec::new)
                            .extend(locations);
                    }
                }
            }
        }
    }
    
    // Count occurrences and create reports
    let mut reports: Vec<SubexpressionReport> = all_expressions
        .into_iter()
        .map(|(pattern, locations)| {
            let count = locations.len();
            let emoji_hash = generate_emoji_hash(&pattern);
            SubexpressionReport {
                pattern: pattern.clone(),
                count,
                emoji_hash,
                locations,
            }
        })
        .collect();
    
    // Sort by frequency (most common first)
    reports.sort_by(|a, b| b.count.cmp(&a.count));
    
    println!("\n🎯 TOP 20 MOST COMMON SUBEXPRESSIONS:");
    println!("=====================================");
    
    for (i, report) in reports.iter().take(20).enumerate() {
        println!("{}. {} {} (Count: {})", 
            i + 1, 
            report.emoji_hash,
            report.pattern,
            report.count
        );
        
        // Show first few locations
        let sample_locations: Vec<_> = report.locations.iter()
            .take(3)
            .collect();
        println!("   Found in: {:?}", sample_locations);
        println!();
    }
    
    // Generate emoji summary
    println!("🎨 EMOJI CONTENT HASH SUMMARY:");
    println!("==============================");
    
    for report in reports.iter().take(10) {
        println!("{} = {} ({}x)", 
            report.emoji_hash, 
            report.pattern.chars().take(50).collect::<String>(),
            report.count
        );
    }
    
    println!("\n📊 ANALYSIS SUMMARY:");
    println!("Unique subexpressions: {}", reports.len());
    println!("Total occurrences: {}", reports.iter().map(|r| r.count).sum::<usize>());
    
    // Save emoji mapping
    let emoji_map: HashMap<String, String> = reports.iter()
        .map(|r| (r.emoji_hash.clone(), r.pattern.clone()))
        .collect();
    
    let emoji_json = serde_json::to_string_pretty(&emoji_map)?;
    fs::write("emoji_subexpression_map.json", emoji_json)?;
    println!("💾 Saved emoji mapping to emoji_subexpression_map.json");
    
    Ok(())
}
