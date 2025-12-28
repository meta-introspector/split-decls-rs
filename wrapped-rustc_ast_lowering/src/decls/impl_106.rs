macro_rules! deps {
    () => {
        ImplTraitPosition!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl std :: fmt :: Display for ImplTraitPosition { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let name = match self { ImplTraitPosition :: Path => "paths" , ImplTraitPosition :: Variable => "the type of variable bindings" , ImplTraitPosition :: Trait => "traits" , ImplTraitPosition :: Bound => "bounds" , ImplTraitPosition :: Generic => "generics" , ImplTraitPosition :: ExternFnParam => "`extern fn` parameters" , ImplTraitPosition :: ClosureParam => "closure parameters" , ImplTraitPosition :: PointerParam => "`fn` pointer parameters" , ImplTraitPosition :: FnTraitParam => "the parameters of `Fn` trait bounds" , ImplTraitPosition :: ExternFnReturn => "`extern fn` return types" , ImplTraitPosition :: ClosureReturn => "closure return types" , ImplTraitPosition :: PointerReturn => "`fn` pointer return types" , ImplTraitPosition :: FnTraitReturn => "the return type of `Fn` trait bounds" , ImplTraitPosition :: GenericDefault => "generic parameter defaults" , ImplTraitPosition :: ConstTy => "const types" , ImplTraitPosition :: StaticTy => "static types" , ImplTraitPosition :: AssocTy => "associated types" , ImplTraitPosition :: FieldTy => "field types" , ImplTraitPosition :: Cast => "cast expression types" , ImplTraitPosition :: ImplSelf => "impl headers" , ImplTraitPosition :: OffsetOf => "`offset_of!` parameters" , } ; write ! (f , "{name}") } }
    };
}

impl_106!();