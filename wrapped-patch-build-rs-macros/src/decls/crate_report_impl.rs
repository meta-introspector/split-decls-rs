macro_rules! crate_report_impl {
    () => {
        # [decl2 (fn , name = "crate_report_impl" , vis = "pub" , hash = "1a2d2b26")] pub fn crate_report_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let crate_path = input_str . value () ; quote ! { { use std :: fs ; let cargo_toml = format ! ("{}/Cargo.toml" , # crate_path) ; let content = fs :: read_to_string (& cargo_toml) . unwrap_or_else (| _ | "[package]\nname = \"unknown\"" . to_string ()) ; let name = content . lines () . find (| line | line . starts_with ("name")) . and_then (| line | line . split ('=') . nth (1)) . map (| s | s . trim () . trim_matches ('"')) . unwrap_or ("unknown") ; let deps = content . lines () . filter (| line | line . contains ("=") && ! line . starts_with ("#")) . filter (| line | ! line . starts_with ("[")) . count () ; let report = format ! ("Crate: {} | Dependencies: {} | Path: {}" , name , deps , # crate_path) ; println ! ("cargo:warning=📋 {}" , report) ; report } } . into () }
    };
}

crate_report_impl!()