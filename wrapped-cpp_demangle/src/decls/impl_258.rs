macro_rules! deps {
    () => {
        DemangleContext!();
        SimpleId!();
        Demangle!();
        Result!();
        ArgScopeStack!();
        DemangleWrite!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for SimpleId where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; self . 0 . demangle (ctx , scope) ? ; if let Some (ref args) = self . 1 { args . demangle (ctx , scope) ? ; } Ok (()) } }
    };
}

impl_258!()