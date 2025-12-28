macro_rules! deps {
    () => {
        QualifiedBuiltin!();
        ArgScopeStack!();
        Result!();
        DemangleContext!();
        DemangleWrite!();
        Demangle!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for QualifiedBuiltin where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; ctx . push_inner (& self . 0) ; self . 1 . demangle (ctx , scope) ? ; if ctx . pop_inner_if (& self . 0) { self . 0 . demangle_as_inner (ctx , scope) ? ; } Ok (()) } }
    };
}

impl_180!();