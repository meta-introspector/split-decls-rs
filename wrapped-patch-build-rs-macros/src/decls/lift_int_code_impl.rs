macro_rules! lift_int_code_impl {
    () => {
        # [decl2 (fn , name = "lift_int_code_impl" , vis = "pub" , hash = "08978ec3")] pub fn lift_int_code_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let block_data = input_str . value () ; quote ! { { println ! ("cargo:warning=⬆️ Lifting blockchain data to Rust code") ; let numbers : Vec < u64 > = # block_data . chars () . filter (| c | c . is_ascii_digit ()) . collect ::< String > () . chars () . collect ::< Vec < _ >> () . chunks (8) . filter_map (| chunk | { let s : String = chunk . iter () . collect () ; s . parse () . ok () }) . collect () ; let lifted_code = format ! ("macro_rules! blockchain_data {{\n    () => {{\n        vec![{}]\n    }};\n}}" , numbers . iter () . map (| n | n . to_string ()) . collect ::< Vec < _ >> () . join (", ")) ; println ! ("cargo:warning=🚀 Lifted {} numbers to macro" , numbers . len ()) ; lifted_code } } . into () }
    };
}

lift_int_code_impl!()