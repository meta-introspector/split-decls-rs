macro_rules! deps {
    () => {
        AbiTags!();
        ArgScopeStack!();
        Demangle!();
        DemangleContext!();
        Result!();
        DemangleWrite!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for AbiTags where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; for tag in & self . 0 { tag . demangle (ctx , scope) ? ; } Ok (()) } }
    };
}

impl_124!();