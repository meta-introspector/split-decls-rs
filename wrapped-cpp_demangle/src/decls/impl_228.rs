macro_rules! deps {
    () => {
        DemangleContext!();
        TemplateTemplateParam!();
        ArgScopeStack!();
        Result!();
        DemangleWrite!();
        Demangle!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for TemplateTemplateParam where W : 'subs + DemangleWrite , { # [inline] fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; self . 0 . demangle (ctx , scope) } }
    };
}

impl_228!()