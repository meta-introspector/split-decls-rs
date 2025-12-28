macro_rules! deps {
    () => {
        AssistContext!();
    };
}

macro_rules! does_pat_variant_nested_or_literal {
    () => {
        deps!();
        pub (crate) fn does_pat_variant_nested_or_literal (ctx : & AssistContext < '_ > , pat : & ast :: Pat) -> bool { check_pat_variant_nested_or_literal_with_depth (ctx , pat , 0) }
    };
}

does_pat_variant_nested_or_literal!();