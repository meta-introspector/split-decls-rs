macro_rules! deps {
    () => {
        ExprOrPatId!();
        PatId!();
        ExprId!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl ExprOrPatId { pub fn as_expr (self) -> Option < ExprId > { match self { Self :: ExprId (v) => Some (v) , _ => None , } } pub fn is_expr (& self) -> bool { matches ! (self , Self :: ExprId (_)) } pub fn as_pat (self) -> Option < PatId > { match self { Self :: PatId (v) => Some (v) , _ => None , } } pub fn is_pat (& self) -> bool { matches ! (self , Self :: PatId (_)) } }
    };
}

impl_215!()