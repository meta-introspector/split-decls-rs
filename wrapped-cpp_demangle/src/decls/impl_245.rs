macro_rules! deps {
    () => {
        Result!();
        ExprPrimary!();
        ArgScopeStack!();
        Expression!();
        DemangleWrite!();
        DemangleContext!();
        FunctionParam!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl Expression { fn demangle_as_subexpr < 'subs , 'prev , 'ctx , W > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result where W : 'subs + DemangleWrite , { let needs_parens = match * self { Expression :: FunctionParam (_) | Expression :: Primary (ExprPrimary :: External (_)) => false , _ => true , } ; if needs_parens { write ! (ctx , "(") ? ; } self . demangle (ctx , scope) ? ; if needs_parens { write ! (ctx , ")") ? ; } Ok (()) } }
    };
}

impl_245!()