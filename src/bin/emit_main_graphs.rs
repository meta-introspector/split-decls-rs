use std::collections::{HashMap, HashSet};
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct ModuleAnalysis {
    defines: HashMap<String, Vec<String>>,
    uses: HashMap<String, Vec<String>>,
    providers: HashMap<String, Vec<String>>,
    consumers: HashMap<String, Vec<String>>,
}

#[derive(Serialize)]
struct MainRoutineGraph {
    main_file: String,
    all_dependencies: Vec<String>,
    dependency_tree: HashMap<String, Vec<String>>,
}

fn main() {
    let analysis: ModuleAnalysis = serde_json::from_str(
        &fs::read_to_string("module_analysis.json").expect("module_analysis.json not found")
    ).expect("Failed to parse module_analysis.json");
    
    let main_routines = find_main_routines(&analysis);
    let mut graphs = HashMap::new();
    
    for main_file in main_routines {
        let graph = build_dependency_graph(&main_file, &analysis);
        graphs.insert(main_file.clone(), graph);
    }
    
    let output = serde_json::to_string_pretty(&graphs).unwrap();
    fs::write("main_dependency_graphs.json", output).unwrap();
    
    println!("Found {} main routines", graphs.len());
    println!("Dependency graphs saved to main_dependency_graphs.json");
}

fn find_main_routines(analysis: &ModuleAnalysis) -> Vec<String> {
    let mut mains = Vec::new();
    
    for (file, symbols) in &analysis.defines {
        for symbol in symbols {
            if symbol.contains("main") || symbol.contains("fn main") {
                mains.push(file.clone());
                break;
            }
        }
    }
    
    mains
}

fn build_dependency_graph(main_file: &str, analysis: &ModuleAnalysis) -> MainRoutineGraph {
    let mut visited = HashSet::new();
    let mut all_deps = Vec::new();
    let mut dep_tree = HashMap::new();
    
    collect_dependencies(main_file, analysis, &mut visited, &mut all_deps, &mut dep_tree);
    
    MainRoutineGraph {
        main_file: main_file.to_string(),
        all_dependencies: all_deps,
        dependency_tree: dep_tree,
    }
}

fn collect_dependencies(
    file: &str,
    analysis: &ModuleAnalysis,
    visited: &mut HashSet<String>,
    all_deps: &mut Vec<String>,
    dep_tree: &mut HashMap<String, Vec<String>>,
) {
    if visited.contains(file) {
        return;
    }
    visited.insert(file.to_string());
    
    if let Some(uses) = analysis.uses.get(file) {
        let mut file_deps = Vec::new();
        
        for symbol in uses {
            if let Some(providers) = analysis.providers.get(symbol) {
                for provider in providers {
                    if provider != file && !file_deps.contains(provider) {
                        file_deps.push(provider.clone());
                        if !all_deps.contains(provider) {
                            all_deps.push(provider.clone());
                        }
                        collect_dependencies(provider, analysis, visited, all_deps, dep_tree);
                    }
                }
            }
        }
        
        if !file_deps.is_empty() {
            dep_tree.insert(file.to_string(), file_deps);
        }
    }
}
