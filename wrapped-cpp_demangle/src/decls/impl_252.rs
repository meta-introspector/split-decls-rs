macro_rules! deps {
    () => {
        DemangleContext!();
        Decltype!();
        DemangleWrite!();
        Demangle!();
        Result!();
        UnresolvedType!();
        ArgScopeStack!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for UnresolvedType where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { UnresolvedType :: Decltype (ref dt) => dt . demangle (ctx , scope) , UnresolvedType :: Template (ref param , ref args) => { if let Some (ref args) = * args { let scope = scope . push (args) ; param . demangle (ctx , scope) ? ; args . demangle (ctx , scope) ? ; } else { param . demangle (ctx , scope) ? ; } Ok (()) } } } }
    };
}

impl_252!();