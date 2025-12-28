macro_rules! deps {
    () => {
        DemangleContext!();
        ClosureTypeName!();
        Result!();
        ArgScopeStack!();
        DemangleWrite!();
        Demangle!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for ClosureTypeName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; write ! (ctx , "{{lambda(") ? ; self . 0 . demangle (ctx , scope) ? ; write ! (ctx , ")#{}}}" , self . 1 . map_or (1 , | n | n + 2)) ? ; Ok (()) } }
    };
}

impl_280!()