macro_rules! variable_references {
    () => {
        fn variable_references < 'a > (msg : & Message < & 'a str >) -> Vec < & 'a str > { let mut refs = vec ! [] ; if let Some (Pattern { elements }) = & msg . value { for elt in elements { if let PatternElement :: Placeable { expression : Expression :: Inline (InlineExpression :: VariableReference { id }) , } = elt { refs . push (id . name) ; } } } for attr in & msg . attributes { for elt in & attr . value . elements { if let PatternElement :: Placeable { expression : Expression :: Inline (InlineExpression :: VariableReference { id }) , } = elt { refs . push (id . name) ; } } } refs }
    };
}

variable_references!();