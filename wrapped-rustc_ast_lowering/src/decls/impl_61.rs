macro_rules! deps {
    () => {
        WillCreateDefIdsVisitor!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < 'v > rustc_ast :: visit :: Visitor < 'v > for WillCreateDefIdsVisitor { type Result = ControlFlow < Span > ; fn visit_anon_const (& mut self , c : & 'v AnonConst) -> Self :: Result { ControlFlow :: Break (c . value . span) } fn visit_item (& mut self , item : & 'v Item) -> Self :: Result { ControlFlow :: Break (item . span) } fn visit_expr (& mut self , ex : & 'v Expr) -> Self :: Result { match ex . kind { ExprKind :: Gen (..) | ExprKind :: ConstBlock (..) | ExprKind :: Closure (..) => { ControlFlow :: Break (ex . span) } _ => walk_expr (self , ex) , } } }
    };
}

impl_61!()