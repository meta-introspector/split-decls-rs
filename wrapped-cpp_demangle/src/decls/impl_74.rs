macro_rules! deps {
    () => {
        ArgScopeStack!();
        DemangleWrite!();
        CloneSuffix!();
        DemangleContext!();
        Result!();
        Demangle!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for CloneSuffix where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; write ! (ctx , " [clone") ? ; self . 0 . demangle (ctx , scope) ? ; for nonnegative in & self . 1 { write ! (ctx , ".{}" , nonnegative) ? ; } write ! (ctx , "]") ? ; Ok (()) } }
    };
}

impl_74!();