macro_rules! extract_decl_impl {
    () => {
        # [decl2 (fn , name = "extract_decl_impl" , vis = "pub" , hash = "e544fc67")] pub fn extract_decl_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let rust_file = input_str . value () ; quote ! { { use std :: fs ; let content = fs :: read_to_string (# rust_file) . unwrap_or_else (| _ | "// File not found" . to_string ()) ; let decls = content . lines () . filter (| line | line . trim_start () . starts_with ("pub fn") || line . trim_start () . starts_with ("fn")) . collect ::< Vec < _ >> () . join ("\n") ; println ! ("cargo:warning=📦 Extracted {} declarations from {}" , decls . lines () . count () , # rust_file) ; decls } } . into () }
    };
}

extract_decl_impl!()