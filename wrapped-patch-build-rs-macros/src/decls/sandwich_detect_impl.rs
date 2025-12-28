macro_rules! sandwich_detect_impl {
    () => {
        # [decl2 (fn , name = "sandwich_detect_impl" , vis = "pub" , hash = "bafc1064")] pub fn sandwich_detect_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let tx_pattern = input_str . value () ; quote ! { { println ! ("cargo:warning=🥪 Detecting sandwich patterns in: {}" , # tx_pattern) ; let is_sandwich = # tx_pattern . contains ("buy") && # tx_pattern . contains ("sell") && # tx_pattern . matches ("->") . count () >= 2 ; let pattern_analysis = if is_sandwich { format ! ("SANDWICH_DETECTED: {}" , # tx_pattern) } else { format ! ("CLEAN_TX: {}" , # tx_pattern) } ; let exclusion_macro = format ! (r#"
macro_rules! exclude_sandwich {{
    ({}) => {{
        compile_error!("Sandwich attack pattern detected and blocked");
    }};
    ($other:expr) => {{
        $other // Allow non-sandwich transactions
    }};
}}
                "# , # tx_pattern) ; println ! ("cargo:warning=🛡️ MEV protection: {}" , if is_sandwich { "BLOCKED" } else { "ALLOWED" }) ; exclusion_macro } } . into () }
    };
}

sandwich_detect_impl!();