macro_rules! deps {
    () => {
        Demangle!();
        UnscopedName!();
        DemangleWrite!();
        Result!();
        DemangleContext!();
        ArgScopeStack!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for UnscopedName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { UnscopedName :: Unqualified (ref unqualified) => unqualified . demangle (ctx , scope) , UnscopedName :: Std (ref std) => { write ! (ctx , "std::") ? ; std . demangle (ctx , scope) } } } }
    };
}

impl_86!();