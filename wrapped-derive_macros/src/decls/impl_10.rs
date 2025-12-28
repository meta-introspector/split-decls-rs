macro_rules! deps {
    () => {
        LabelAttr!();
        LabelValue!();
        SvalAttribute!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl SvalAttribute for LabelAttr { type Result = LabelValue ; fn try_from_expr (& self , expr : & Expr) -> Option < Self :: Result > { match expr { Expr :: Lit (lit) => Some (self . from_lit (& lit . lit)) , Expr :: Path (path) => Some (LabelValue :: Ident (quote ! (# path))) , _ => None , } } fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Str (ref s) = lit { LabelValue :: Const (s . value ()) } else { panic ! ("unexpected value") } } }
    };
}

impl_10!()