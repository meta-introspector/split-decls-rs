macro_rules! deps {
    () => {
        ConstEvalError!();
        HirDatabase!();
        DisplayTarget!();
    };
}

macro_rules! impl_444 {
    () => {
        deps!();
        impl ConstEvalError < '_ > { pub fn pretty_print (& self , f : & mut String , db : & dyn HirDatabase , span_formatter : impl Fn (span :: FileId , span :: TextRange) -> String , display_target : DisplayTarget ,) -> std :: result :: Result < () , std :: fmt :: Error > { match self { ConstEvalError :: MirLowerError (e) => { e . pretty_print (f , db , span_formatter , display_target) } ConstEvalError :: MirEvalError (e) => { e . pretty_print (f , db , span_formatter , display_target) } } } }
    };
}

impl_444!()