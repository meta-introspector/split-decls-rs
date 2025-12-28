macro_rules! real_rustc_analysis_impl {
    () => {
        # [decl (fn , name = "real_rustc_analysis_impl" , vis = "pub" , hash = "3a7fcdcf")] pub fn real_rustc_analysis_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let source_path = input_str . value () ; quote ! { { use std :: fs ; use std :: path :: Path ; println ! ("cargo:warning=📊 Real rustc analysis: {}" , # source_path) ; let analysis = if Path :: new (# source_path) . exists () { let mut stats = std :: collections :: HashMap :: new () ; let mut file_count = 0 ; let mut total_lines = 0 ; let mut function_count = 0 ; let mut struct_count = 0 ; let mut enum_count = 0 ; let mut trait_count = 0 ; fn walk_dir (dir : & Path , stats : & mut std :: collections :: HashMap < String , usize >, file_count : & mut usize , total_lines : & mut usize , function_count : & mut usize , struct_count : & mut usize , enum_count : & mut usize , trait_count : & mut usize) -> std :: io :: Result < () > { if dir . is_dir () { for entry in fs :: read_dir (dir) ? { let entry = entry ?; let path = entry . path () ; if path . is_dir () { walk_dir (& path , stats , file_count , total_lines , function_count , struct_count , enum_count , trait_count) ?; } else if path . extension () . map_or (false , | ext | ext == "rs") { * file_count += 1 ; if let Ok (content) = fs :: read_to_string (& path) { * total_lines += content . lines () . count () ; * function_count += content . matches ("fn ") . count () ; * struct_count += content . matches ("struct ") . count () ; * enum_count += content . matches ("enum ") . count () ; * trait_count += content . matches ("trait ") . count () ; } } } } Ok (()) } let _ = walk_dir (Path :: new (# source_path) , & mut stats , & mut file_count , & mut total_lines , & mut function_count , & mut struct_count , & mut enum_count , & mut trait_count) ; format ! (r#"
📊 REAL RUSTC SOURCE ANALYSIS

📁 Source Path: {}
✅ Path Exists: true

📈 File Statistics:
- Rust files (.rs): {}
- Total lines of code: {}
- Average lines per file: {:.1}

🔍 Code Element Counts:
- Functions (fn): {}
- Structs (struct): {}
- Enums (enum): {}
- Traits (trait): {}

📊 Real Ratios:
- Functions per file: {:.1}
- Structs per file: {:.1}
- Enums per file: {:.1}
- Traits per file: {:.1}

🎯 VERIFICATION: All numbers derived from actual file analysis
📍 Source: {}
🔍 Method: Directory traversal + pattern counting
                    "# , # source_path , file_count , total_lines , if file_count > 0 { total_lines as f64 / file_count as f64 } else { 0.0 } , function_count , struct_count , enum_count , trait_count , if file_count > 0 { function_count as f64 / file_count as f64 } else { 0.0 } , if file_count > 0 { struct_count as f64 / file_count as f64 } else { 0.0 } , if file_count > 0 { enum_count as f64 / file_count as f64 } else { 0.0 } , if file_count > 0 { trait_count as f64 / file_count as f64 } else { 0.0 } , # source_path) } else { format ! (r#"
❌ REAL RUSTC SOURCE ANALYSIS FAILED

📁 Source Path: {}
❌ Path Exists: false

🔍 To get real rustc source:
1. rustup component add rust-src
2. Or: nix-shell -p rustc.src
3. Or: curl -L https://github.com/rust-lang/rust/archive/master.tar.gz | tar xz

📍 Current directory: {:?}
🎯 Need valid source path for real analysis
                    "# , # source_path , std :: env :: current_dir () . unwrap_or_else (| _ | std :: path :: PathBuf :: from ("unknown"))) } ; analysis } } . into () }
    };
}

real_rustc_analysis_impl!()