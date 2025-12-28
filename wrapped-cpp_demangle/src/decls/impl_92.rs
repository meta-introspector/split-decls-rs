macro_rules! deps {
    () => {
        Result!();
        ArgScopeStack!();
        UnscopedTemplateName!();
        Demangle!();
        DemangleContext!();
        DemangleWrite!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for UnscopedTemplateName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; self . 0 . demangle (ctx , scope) } }
    };
}

impl_92!()