macro_rules! deps {
    () => {
        Result!();
        Demangle!();
        DemangleWrite!();
        ArgScopeStack!();
        ExceptionSpec!();
        DemangleContext!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for ExceptionSpec where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { ExceptionSpec :: NoExcept => write ! (ctx , "noexcept") , ExceptionSpec :: Computed (ref expr) => { write ! (ctx , "noexcept(") ? ; expr . demangle (ctx , scope) ? ; write ! (ctx , ")") } } } }
    };
}

impl_184!();