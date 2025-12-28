macro_rules! deps {
    () => {
        IndexAttr!();
        IndexValue!();
        SvalAttribute!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl SvalAttribute for IndexAttr { type Result = IndexValue ; fn try_from_expr (& self , expr : & Expr) -> Option < Self :: Result > { match expr { Expr :: Unary (ExprUnary { op : UnOp :: Neg (_) , expr , .. }) => { if let Expr :: Lit (ref lit) = * * expr { Some (IndexValue :: Const (- (self . const_from_lit (& lit . lit)))) } else { None } } Expr :: Lit (lit) => Some (IndexValue :: Const (self . const_from_lit (& lit . lit))) , Expr :: Path (path) => Some (IndexValue :: Ident (quote ! (# path))) , _ => None , } } fn from_lit (& self , lit : & Lit) -> Self :: Result { IndexValue :: Const (self . const_from_lit (lit)) } }
    };
}

impl_14!()