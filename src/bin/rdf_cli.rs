use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RdfState {
    last_query: String,
    query_history: Vec<String>,
    bookmarks: HashMap<String, String>,
}

struct RdfInterpreter {
    triples: Vec<(String, String, String)>,
    predicates: HashMap<String, usize>,
    subjects: HashMap<String, usize>,
    objects: HashMap<String, usize>,
    state: RdfState,
}

impl RdfInterpreter {
    fn new() -> Self {
        let state = Self::load_state().unwrap_or_else(|_| RdfState {
            last_query: String::new(),
            query_history: Vec::new(),
            bookmarks: HashMap::new(),
        });
        
        Self {
            triples: Vec::new(),
            predicates: HashMap::new(),
            subjects: HashMap::new(),
            objects: HashMap::new(),
            state,
        }
    }
    
    fn load_state() -> Result<RdfState> {
        let content = fs::read_to_string("rdf_state.json")?;
        Ok(serde_json::from_str(&content)?)
    }
    
    fn save_state(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.state)?;
        fs::write("rdf_state.json", json)?;
        Ok(())
    }
    
    fn load_kb(&mut self, file: &str) -> Result<()> {
        let content = fs::read_to_string(file)?;
        
        for line in content.lines() {
            if let Some((s, p, o)) = self.parse_line(line) {
                self.triples.push((s.clone(), p.clone(), o.clone()));
                *self.predicates.entry(p).or_insert(0) += 1;
                *self.subjects.entry(s).or_insert(0) += 1;
                *self.objects.entry(o).or_insert(0) += 1;
            }
        }
        
        Ok(())
    }
    
    fn parse_line(&self, line: &str) -> Option<(String, String, String)> {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() || line.starts_with('@') || line.starts_with('<') {
            return None;
        }
        
        // Parse "subject predicate object ;"
        if line.contains(" ; ") || line.ends_with(" .") {
            let clean_line = line.replace(" ;", "").replace(" .", "");
            let parts: Vec<&str> = clean_line.split_whitespace().collect();
            
            if parts.len() >= 3 {
                return Some((
                    parts[0].to_string(),
                    parts[1].to_string(), 
                    parts[2..].join(" ").trim_matches('"').to_string()
                ));
            }
        }
        
        None
    }
    
    fn run_cli(&mut self) {
        println!("🔍 RDF Query Interface");
        println!("======================");
        println!("Commands:");
        println!("  predicates - Show predicate counts");
        println!("  subjects - Show subject counts");  
        println!("  objects - Show object counts");
        println!("  query <predicate> - Find triples with predicate");
        println!("  bookmark <name> <query> - Save query as bookmark");
        println!("  run <bookmark> - Run saved bookmark");
        println!("  history - Show query history");
        println!("  last - Repeat last query");
        println!("  stats - Show statistics");
        println!("  save - Save current state");
        println!("  quit - Exit");
        println!();
        
        loop {
            print!("rdf> ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                break;
            }
            
            let input = input.trim();
            if input == "quit" || input == "exit" {
                let _ = self.save_state();
                break;
            }
            
            if !input.is_empty() {
                self.state.query_history.push(input.to_string());
                self.state.last_query = input.to_string();
            }
            
            self.handle_command(input);
        }
    }
    
    fn handle_command(&mut self, cmd: &str) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        
        match parts.get(0) {
            Some(&"predicates") => self.show_predicates(),
            Some(&"subjects") => self.show_subjects(),
            Some(&"objects") => self.show_objects(),
            Some(&"query") => {
                if let Some(predicate) = parts.get(1) {
                    self.query_predicate(predicate);
                } else {
                    println!("Usage: query <predicate>");
                }
            }
            Some(&"bookmark") => {
                if parts.len() >= 3 {
                    let name = parts[1];
                    let query = parts[2..].join(" ");
                    self.state.bookmarks.insert(name.to_string(), query);
                    println!("📌 Bookmarked '{}' as '{}'", parts[1], name);
                } else {
                    println!("Usage: bookmark <name> <query>");
                }
            }
            Some(&"run") => {
                if let Some(name) = parts.get(1) {
                    let name = name.to_string();
                    if let Some(query) = self.state.bookmarks.get(&name).cloned() {
                        println!("🔖 Running bookmark '{}': {}", name, query);
                        self.handle_command(&query);
                    } else {
                        println!("❌ Bookmark '{}' not found", name);
                    }
                } else {
                    println!("Usage: run <bookmark>");
                }
            }
            Some(&"history") => self.show_history(),
            Some(&"last") => {
                if !self.state.last_query.is_empty() {
                    println!("🔄 Repeating: {}", self.state.last_query);
                    let last = self.state.last_query.clone();
                    self.handle_command(&last);
                } else {
                    println!("❌ No previous query");
                }
            }
            Some(&"save") => {
                if self.save_state().is_ok() {
                    println!("💾 State saved");
                } else {
                    println!("❌ Failed to save state");
                }
            }
            Some(&"stats") => self.show_stats(),
            _ => println!("Unknown command: {}", cmd),
        }
    }
    
    fn show_predicates(&self) {
        let mut preds: Vec<_> = self.predicates.iter().collect();
        preds.sort_by(|a, b| b.1.cmp(a.1));
        
        println!("📊 Top Predicates:");
        for (i, (pred, count)) in preds.iter().take(15).enumerate() {
            println!("   {}. {} ({})", i+1, pred, count);
        }
    }
    
    fn show_subjects(&self) {
        let mut subjs: Vec<_> = self.subjects.iter().collect();
        subjs.sort_by(|a, b| b.1.cmp(a.1));
        
        println!("👤 Top Subjects:");
        for (i, (subj, count)) in subjs.iter().take(10).enumerate() {
            println!("   {}. {} ({})", i+1, subj, count);
        }
    }
    
    fn show_objects(&self) {
        let mut objs: Vec<_> = self.objects.iter().collect();
        objs.sort_by(|a, b| b.1.cmp(a.1));
        
        println!("🎯 Top Objects:");
        for (i, (obj, count)) in objs.iter().take(10).enumerate() {
            println!("   {}. {} ({})", i+1, obj, count);
        }
    }
    
    fn query_predicate(&self, predicate: &str) {
        let matches: Vec<_> = self.triples.iter()
            .filter(|(_, p, _)| p.contains(predicate))
            .collect();
        
        println!("🔍 Query results for '{}': {} matches", predicate, matches.len());
        for (i, (s, p, o)) in matches.iter().take(10).enumerate() {
            println!("   {}. {} {} {}", i+1, s, p, o);
        }
        
        if matches.len() > 10 {
            println!("   ... and {} more", matches.len() - 10);
        }
    }
    
    fn show_history(&self) {
        println!("📜 Query History:");
        for (i, query) in self.state.query_history.iter().rev().take(10).enumerate() {
            println!("   {}. {}", i+1, query);
        }
        
        if !self.state.bookmarks.is_empty() {
            println!("\n📌 Bookmarks:");
            for (name, query) in &self.state.bookmarks {
                println!("   {}: {}", name, query);
            }
        }
    }
    
    fn show_stats(&self) {
        println!("📊 Knowledge Base Statistics:");
        println!("   Total triples: {}", self.triples.len());
        println!("   Unique predicates: {}", self.predicates.len());
        println!("   Unique subjects: {}", self.subjects.len());
        println!("   Unique objects: {}", self.objects.len());
    }
}

fn main() -> Result<()> {
    let mut interpreter = RdfInterpreter::new();
    
    println!("🚀 Loading LMDFB Knowledge Base...");
    interpreter.load_kb("rustc_lmdfb_knowledge_base.owl")?;
    
    println!("✅ Knowledge base loaded successfully!");
    interpreter.show_stats();
    
    // Show top predicates immediately
    interpreter.show_predicates();
    
    // Start interactive CLI
    interpreter.run_cli();
    
    Ok(())
}
