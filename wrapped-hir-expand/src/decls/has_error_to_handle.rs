macro_rules! has_error_to_handle {
    () => {
        fn has_error_to_handle (node : & SyntaxNode) -> bool { has_error (node) || node . children () . any (| c | ! can_handle_error (& c) && has_error_to_handle (& c)) }
    };
}

has_error_to_handle!();