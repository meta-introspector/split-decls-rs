macro_rules! deps {
    () => {
        ArgScopeStack!();
        DemangleContext!();
        DemangleWrite!();
        AbiTag!();
        Demangle!();
        Result!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for AbiTag where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; write ! (ctx , "[abi:") ? ; self . 0 . demangle (ctx , scope) ? ; write ! (ctx , "]") } }
    };
}

impl_127!();