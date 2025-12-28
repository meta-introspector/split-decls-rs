macro_rules! variant_to_string {
    () => {
        fn variant_to_string (var : & ast :: Variant) -> String { to_string (| s | s . print_variant (var)) }
    };
}

variant_to_string!();