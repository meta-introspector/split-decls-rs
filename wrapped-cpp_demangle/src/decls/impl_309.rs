macro_rules! deps {
    () => {
        DemangleContext!();
        FoldExpr!();
        DemangleWrite!();
        ArgScopeStack!();
        Result!();
        Demangle!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for FoldExpr where W : 'subs + DemangleWrite , { # [inline] fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match self { FoldExpr :: UnaryLeft (ref operator , ref expr) => { write ! (ctx , "(...") ? ; operator . demangle (ctx , scope) ? ; expr . demangle_as_subexpr (ctx , scope) ? ; write ! (ctx , ")") } FoldExpr :: UnaryRight (ref operator , ref expr) => { write ! (ctx , "(") ? ; expr . demangle_as_subexpr (ctx , scope) ? ; operator . demangle (ctx , scope) ? ; write ! (ctx , "...)") } FoldExpr :: BinaryLeft (ref operator , ref expr1 , ref expr2) | FoldExpr :: BinaryRight (ref operator , ref expr1 , ref expr2) => { write ! (ctx , "(") ? ; expr1 . demangle_as_subexpr (ctx , scope) ? ; operator . demangle (ctx , scope) ? ; write ! (ctx , "...") ? ; operator . demangle (ctx , scope) ? ; expr2 . demangle_as_subexpr (ctx , scope) ? ; write ! (ctx , ")") } } } }
    };
}

impl_309!()