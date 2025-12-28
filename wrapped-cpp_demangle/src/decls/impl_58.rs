macro_rules! deps {
    () => {
        DemangleContext!();
        Demangle!();
        DemangleWrite!();
        ArgScopeStack!();
        Result!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for FunctionArgListAndReturnType where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { FunctionArgSlice :: new (& self . 0 [1 ..]) . demangle (ctx , scope) } }
    };
}

impl_58!();