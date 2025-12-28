macro_rules! deps {
    () => {
        DemangleContext!();
        Demangle!();
        Decltype!();
        DemangleNodeType!();
        Result!();
        ArgScopeStack!();
        TemplateParam!();
        Expression!();
        DemangleWrite!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for Decltype where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; ctx . push_demangle_node (DemangleNodeType :: TemplateParam) ; let ret = match * self { Decltype :: Expression (ref expr) | Decltype :: IdExpression (ref expr) => { write ! (ctx , "decltype (") ? ; expr . demangle (ctx , scope) ? ; write ! (ctx , ")") ? ; Ok (()) } } ; ctx . pop_demangle_node () ; ret } }
    };
}

impl_197!()