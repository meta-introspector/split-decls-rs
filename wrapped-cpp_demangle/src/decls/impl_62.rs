macro_rules! deps {
    () => {
        DemangleWrite!();
        Result!();
        NonSubstitution!();
        ArgScopeStack!();
        Demangle!();
        DemangleContext!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for NonSubstitution where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { ctx . subs . non_substitution (self . 0) . demangle (ctx , scope) } }
    };
}

impl_62!();