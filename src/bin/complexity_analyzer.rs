use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;
use syn::{File, Item, ItemStruct, ItemEnum, Fields};

#[derive(Debug, Clone)]
struct ComplexityReport {
    name: String,
    complexity: u8,
    field_count: usize,
    variant_count: usize,
    nested_depth: u8,
    file_path: String,
}

fn analyze_complexity(item: &Item, file_path: &str) -> Option<ComplexityReport> {
    match item {
        Item::Struct(s) => analyze_struct_complexity(s, file_path),
        Item::Enum(e) => analyze_enum_complexity(e, file_path),
        _ => None,
    }
}

fn analyze_struct_complexity(s: &ItemStruct, file_path: &str) -> Option<ComplexityReport> {
    let field_count = match &s.fields {
        Fields::Named(fields) => fields.named.len(),
        Fields::Unnamed(fields) => fields.unnamed.len(),
        Fields::Unit => 0,
    };
    
    let nested_depth = calculate_nested_depth(&s.fields);
    let complexity = calculate_complexity_score(field_count, 0, nested_depth);
    
    Some(ComplexityReport {
        name: s.ident.to_string(),
        complexity,
        field_count,
        variant_count: 0,
        nested_depth,
        file_path: file_path.to_string(),
    })
}

fn analyze_enum_complexity(e: &syn::ItemEnum, file_path: &str) -> Option<ComplexityReport> {
    let variant_count = e.variants.len();
    let max_fields = e.variants.iter()
        .map(|v| match &v.fields {
            Fields::Named(fields) => fields.named.len(),
            Fields::Unnamed(fields) => fields.unnamed.len(),
            Fields::Unit => 0,
        })
        .max()
        .unwrap_or(0);
    
    let nested_depth = e.variants.iter()
        .map(|v| calculate_nested_depth(&v.fields))
        .max()
        .unwrap_or(0);
    
    let complexity = calculate_complexity_score(max_fields, variant_count, nested_depth);
    
    Some(ComplexityReport {
        name: e.ident.to_string(),
        complexity,
        field_count: max_fields,
        variant_count,
        nested_depth,
        file_path: file_path.to_string(),
    })
}

fn calculate_nested_depth(fields: &Fields) -> u8 {
    // Simplified depth calculation - count generic parameters and nested types
    match fields {
        Fields::Named(fields) => {
            fields.named.iter()
                .map(|f| count_type_complexity(&f.ty))
                .max()
                .unwrap_or(0)
        },
        Fields::Unnamed(fields) => {
            fields.unnamed.iter()
                .map(|f| count_type_complexity(&f.ty))
                .max()
                .unwrap_or(0)
        },
        Fields::Unit => 0,
    }
}

fn count_type_complexity(ty: &syn::Type) -> u8 {
    match ty {
        syn::Type::Path(path) => {
            let segments = &path.path.segments;
            let base_complexity = if segments.len() > 2 { 2 } else { 1 };
            
            // Add complexity for generics
            let generic_complexity = segments.iter()
                .map(|seg| match &seg.arguments {
                    syn::PathArguments::AngleBracketed(args) => args.args.len() as u8,
                    _ => 0,
                })
                .sum::<u8>();
            
            base_complexity + generic_complexity
        },
        syn::Type::Reference(_) => 1,
        syn::Type::Tuple(tuple) => tuple.elems.len() as u8,
        _ => 1,
    }
}

fn calculate_complexity_score(field_count: usize, variant_count: usize, nested_depth: u8) -> u8 {
    let base_score = match field_count {
        0..=2 => 1,
        3..=4 => 2,
        5..=6 => 3,
        7..=8 => 4,
        9..=10 => 5,
        11..=12 => 6,
        _ => 7,
    };
    
    let variant_bonus = match variant_count {
        0..=2 => 0,
        3..=5 => 1,
        6..=10 => 2,
        _ => 3,
    };
    
    let depth_bonus = nested_depth.min(3);
    
    (base_score + variant_bonus + depth_bonus).min(10)
}

fn analyze_rust_file(file_path: &Path) -> Result<Vec<ComplexityReport>> {
    let content = fs::read_to_string(file_path)?;
    let syntax_tree: File = syn::parse_file(&content)?;
    
    let mut reports = Vec::new();
    for item in syntax_tree.items {
        if let Some(report) = analyze_complexity(&item, file_path.to_string_lossy().as_ref()) {
            reports.push(report);
        }
    }
    
    Ok(reports)
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
    println!("🔍 COMPLEXITY ANALYZER - Finding Most Complex Objects");
    println!("====================================================");
    
    let search_paths = vec![
        "submodules/rust/compiler",
        "submodules/rust/library", 
        "src",
    ];
    
    let mut all_reports = Vec::new();
    
    for search_path in search_paths {
        let path = Path::new(search_path);
        if path.exists() {
            println!("\n📂 Analyzing: {}", search_path);
            let rust_files = find_rust_files(path);
            println!("   Found {} Rust files", rust_files.len());
            
            for file_path in rust_files.iter().take(100) { // Limit for performance
                if let Ok(reports) = analyze_rust_file(file_path) {
                    all_reports.extend(reports);
                }
            }
        }
    }
    
    // Sort by complexity (highest first)
    all_reports.sort_by(|a, b| b.complexity.cmp(&a.complexity));
    
    println!("\n🎯 TOP 20 MOST COMPLEX OBJECTS:");
    println!("================================");
    
    for (i, report) in all_reports.iter().take(20).enumerate() {
        println!("{}. {} (Complexity: {})", 
            i + 1, 
            report.name, 
            report.complexity
        );
        println!("   Fields: {}, Variants: {}, Depth: {}", 
            report.field_count, 
            report.variant_count, 
            report.nested_depth
        );
        println!("   File: {}", report.file_path);
        println!();
    }
    
    // Find complexity level 7+ objects
    let complex_objects: Vec<_> = all_reports.iter()
        .filter(|r| r.complexity >= 7)
        .collect();
    
    println!("🔥 COMPLEXITY LEVEL 7+ OBJECTS:");
    println!("===============================");
    
    for report in &complex_objects {
        println!("• {} (Level {})", report.name, report.complexity);
        println!("  {} fields, {} variants, depth {}", 
            report.field_count, report.variant_count, report.nested_depth);
    }
    
    println!("\n📊 SUMMARY:");
    println!("Total objects analyzed: {}", all_reports.len());
    println!("Complexity 7+ objects: {}", complex_objects.len());
    
    Ok(())
}
