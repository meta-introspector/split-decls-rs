macro_rules! deps {
    () => {
        Result!();
        Demangle!();
        DemangleContext!();
        DemangleWrite!();
        ArgScopeStack!();
        UnresolvedQualifierLevel!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for UnresolvedQualifierLevel where W : 'subs + DemangleWrite , { # [inline] fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; self . 0 . demangle (ctx , scope) } }
    };
}

impl_255!();