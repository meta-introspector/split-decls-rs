macro_rules! deps {
    () => {
        DemangleWrite!();
        Demangle!();
        FunctionParam!();
        DemangleContext!();
        ArgScopeStack!();
        Result!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for FunctionParam where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match self . 2 { None => write ! (ctx , "this") , Some (i) => write ! (ctx , "{{parm#{}}}" , i + 1) , } } }
    };
}

impl_231!()