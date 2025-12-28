macro_rules! compiler_macro_impl {
    () => {
        # [decl (fn , name = "compiler_macro_impl" , vis = "pub" , hash = "c38288b9")] pub fn compiler_macro_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let compiler = input_str . value () ; quote ! { { println ! ("cargo:warning=⚙️ Creating compiler macro: {}" , # compiler) ; let compiler_name = # compiler . replace ("-" , "_") ; let macro_code = format ! (r###"
macro_rules! {}_compile {{
    ($source:expr) => {{
        use std::process::Command;
        
        let result = Command::new("{}")
            .arg("-")
            .stdin(std::process::Stdio::piped())
            .output();
            
        match result {{
            Ok(output) => format!("✅ {} compiled", "{}"),
            Err(_) => format!("❌ {} not found", "{}")
        }}
    }};
}}
            "### , compiler_name , # compiler , # compiler , # compiler) ; macro_code } } . into () }
    };
}

compiler_macro_impl!()