macro_rules! NonBindingLetSub {
    () => {
        pub (crate) struct NonBindingLetSub { pub suggestion : Span , pub drop_fn_start_end : Option < (Span , Span) > , pub is_assign_desugar : bool , }
    };
}

NonBindingLetSub!()