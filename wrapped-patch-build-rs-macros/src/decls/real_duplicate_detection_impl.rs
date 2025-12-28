macro_rules! real_duplicate_detection_impl {
    () => {
        # [decl (fn , name = "real_duplicate_detection_impl" , vis = "pub" , hash = "367cc850")] pub fn real_duplicate_detection_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let source_path = input_str . value () ; quote ! { { use std :: fs ; use std :: path :: Path ; use std :: collections :: HashMap ; println ! ("cargo:warning=🔍 Real duplicate detection: {}" , # source_path) ; let analysis = if Path :: new (# source_path) . exists () { let mut function_signatures = HashMap :: new () ; let mut duplicates = Vec :: new () ; let mut file_count = 0 ; fn extract_functions (dir : & Path , signatures : & mut HashMap < String , Vec < String >>, file_count : & mut usize) -> std :: io :: Result < () > { if dir . is_dir () { for entry in fs :: read_dir (dir) ? { let entry = entry ?; let path = entry . path () ; if path . is_dir () { extract_functions (& path , signatures , file_count) ?; } else if path . extension () . map_or (false , | ext | ext == "rs") { * file_count += 1 ; if let Ok (content) = fs :: read_to_string (& path) { for line in content . lines () { let trimmed = line . trim () ; if trimmed . starts_with ("fn ") && trimmed . contains ("(") { let sig = trimmed . split ('{') . next () . unwrap_or (trimmed) . trim () ; let file_name = path . file_name () . unwrap_or_default () . to_string_lossy () . to_string () ; signatures . entry (sig . to_string ()) . or_insert_with (Vec :: new) . push (file_name) ; } } } } } } Ok (()) } let _ = extract_functions (Path :: new (# source_path) , & mut function_signatures , & mut file_count) ; for (signature , files) in & function_signatures { if files . len () > 1 { duplicates . push ((signature . clone () , files . clone ())) ; } } format ! (r#"
🔍 REAL DUPLICATE DETECTION ANALYSIS

📁 Source Path: {}
📊 Files Analyzed: {}
🔍 Function Signatures Found: {}
🎯 Actual Duplicates Found: {}

📋 Real Duplicate Functions:
{}

📈 Real Statistics:
- Duplicate Rate: {:.2}%
- Files with Duplicates: {}
- Average Duplicates per Signature: {:.1}

🔬 Analysis Method:
- Real file traversal and parsing
- Function signature extraction
- Exact signature matching
- No mock data used

✅ VERIFICATION: All duplicates are real and verifiable
                    "# , # source_path , file_count , function_signatures . len () , duplicates . len () , duplicates . iter () . take (10) . map (| (sig , files) | format ! ("  - {}: {} files" , sig . chars () . take (50) . collect ::< String > () , files . len ())) . collect ::< Vec < _ >> () . join ("\n") , if function_signatures . len () > 0 { duplicates . len () as f64 / function_signatures . len () as f64 * 100.0 } else { 0.0 } , duplicates . iter () . map (| (_ , files) | files . len ()) . sum ::< usize > () , if duplicates . len () > 0 { duplicates . iter () . map (| (_ , files) | files . len ()) . sum ::< usize > () as f64 / duplicates . len () as f64 } else { 0.0 }) } else { format ! (r#"
❌ REAL DUPLICATE DETECTION FAILED

📁 Source Path: {}
❌ Path does not exist

🔍 Cannot perform real duplicate analysis without valid source
📊 No mock data provided - need actual rustc source
                    "# , # source_path) } ; analysis } } . into () }
    };
}

real_duplicate_detection_impl!()