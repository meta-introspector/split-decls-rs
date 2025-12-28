macro_rules! deps {
    () => {
        DemangleWrite!();
        DemangleContext!();
        Result!();
        LambdaSig!();
        ArgScopeStack!();
    };
}

macro_rules! impl_285 {
    () => {
        deps!();
        impl LambdaSig { fn demangle_args < 'subs , 'prev , 'ctx , W > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result where W : 'subs + DemangleWrite , { let mut need_comma = false ; for ty in & self . 0 { if need_comma { write ! (ctx , ", ") ? ; } ty . demangle (ctx , scope) ? ; need_comma = true ; } Ok (()) } }
    };
}

impl_285!()