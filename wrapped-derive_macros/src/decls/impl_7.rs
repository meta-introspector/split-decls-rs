macro_rules! deps {
    () => {
        DataTagAttr!();
        SvalAttribute!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl SvalAttribute for DataTagAttr { type Result = syn :: Path ; fn try_from_expr (& self , expr : & Expr) -> Option < Self :: Result > { match expr { Expr :: Lit (lit) => Some (self . from_lit (& lit . lit)) , Expr :: Path (path) => Some (path . path . clone ()) , _ => None , } } fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Str (ref s) = lit { s . parse () . expect ("invalid value") } else { panic ! ("unexpected value") } } }
    };
}

impl_7!()