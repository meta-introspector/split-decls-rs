macro_rules! deps {
    () => {
        UnnamedTypeName!();
        DemangleContext!();
        Result!();
        Demangle!();
        DemangleWrite!();
        ArgScopeStack!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for UnnamedTypeName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; write ! (ctx , "{{unnamed type#{}}}" , self . 0 . map_or (1 , | n | n + 1)) ? ; Ok (()) } }
    };
}

impl_205!();