macro_rules! deps {
    () => {
        Qualified!();
        PathCompletionCtx!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl PathCompletionCtx < '_ > { pub (crate) fn is_trivial_path (& self) -> bool { matches ! (self , PathCompletionCtx { has_call_parens : false , has_macro_bang : false , qualified : Qualified :: No , parent : None , has_type_args : false , .. }) } }
    };
}

impl_105!()