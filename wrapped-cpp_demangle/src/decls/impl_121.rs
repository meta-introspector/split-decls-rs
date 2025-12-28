macro_rules! deps {
    () => {
        Demangle!();
        SourceName!();
        DemangleContext!();
        ArgScopeStack!();
        Result!();
        DemangleWrite!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for SourceName where W : 'subs + DemangleWrite , { # [inline] fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; self . 0 . demangle (ctx , scope) } }
    };
}

impl_121!()