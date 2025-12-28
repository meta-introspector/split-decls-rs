macro_rules! deps {
    () => {
        Demangle!();
        ArgScopeStack!();
        Result!();
        DemangleContext!();
        DemangleWrite!();
        CvQualifiers!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for CvQualifiers where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; if self . const_ { ctx . ensure_space () ? ; write ! (ctx , "const") ? ; } if self . volatile { ctx . ensure_space () ? ; write ! (ctx , "volatile") ? ; } if self . restrict { ctx . ensure_space () ? ; write ! (ctx , "restrict") ? ; } Ok (()) } }
    };
}

impl_167!();