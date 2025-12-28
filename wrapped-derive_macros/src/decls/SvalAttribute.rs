macro_rules! deps {
    () => {
        RawAttribute!();
    };
}

macro_rules! SvalAttribute {
    () => {
        deps!();
        pub (crate) trait SvalAttribute : RawAttribute { type Result : 'static ; fn try_from_expr (& self , expr : & Expr) -> Option < Self :: Result > { if let Expr :: Lit (lit) = expr { Some (self . from_lit (& lit . lit)) } else { None } } fn from_lit (& self , lit : & Lit) -> Self :: Result ; }
    };
}

SvalAttribute!();