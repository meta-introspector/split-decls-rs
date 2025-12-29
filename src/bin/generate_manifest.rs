use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MainRoutineGraph {
    main_file: String,
    all_dependencies: Vec<String>,
    dependency_tree: HashMap<String, Vec<String>>,
}

#[derive(Serialize)]
struct ExecutableManifest {
    main_routines: HashMap<String, Vec<String>>,
    total_files: usize,
    total_routines: usize,
}

fn main() {
    let graphs: HashMap<String, MainRoutineGraph> = serde_json::from_str(
        &fs::read_to_string("main_dependency_graphs.json").expect("main_dependency_graphs.json not found")
    ).expect("Failed to parse main_dependency_graphs.json");
    
    let mut manifest = ExecutableManifest {
        main_routines: HashMap::new(),
        total_files: 0,
        total_routines: graphs.len(),
    };
    
    for (main_name, graph) in graphs {
        let mut files_needed = vec![graph.main_file.clone()];
        files_needed.extend(graph.all_dependencies);
        
        manifest.total_files += files_needed.len();
        manifest.main_routines.insert(main_name, files_needed);
    }
    
    let output = serde_json::to_string_pretty(&manifest).unwrap();
    fs::write("executable_manifest.json", output).unwrap();
    
    println!("Generated manifest for {} main routines", manifest.total_routines);
    println!("Total files needed: {}", manifest.total_files);
    println!("Manifest saved to executable_manifest.json");
}
