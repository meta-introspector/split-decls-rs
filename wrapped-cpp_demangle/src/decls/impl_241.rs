macro_rules! deps {
    () => {
        DemangleContext!();
        MemberName!();
        Demangle!();
        DemangleWrite!();
        ArgScopeStack!();
        Result!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for MemberName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; let needs_parens = self . 0 . get_template_args (ctx . subs) . is_some () ; if needs_parens { write ! (ctx , "(") ? ; } self . 0 . demangle (ctx , scope) ? ; if needs_parens { write ! (ctx , ")") ? ; } Ok (()) } }
    };
}

impl_241!();