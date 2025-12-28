macro_rules! deps {
    () => {
        ArgScopeStack!();
        DemangleWrite!();
        DemangleContext!();
        Demangle!();
        Result!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for FunctionArgList where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { FunctionArgSlice :: new (& self . 0 [..]) . demangle (ctx , scope) } }
    };
}

impl_56!()