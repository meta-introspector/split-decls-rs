macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_411 {
    () => {
        deps!();
        impl < G : GraphBase > Label < G > { fn is_outer (& self) -> bool { self != & Label :: None && ! matches ! (self , Label :: Flag (_)) } fn is_inner (& self) -> bool { ! self . is_outer () } fn to_vertex (& self) -> Option < G :: NodeId > { match * self { Label :: Vertex (v) => Some (v) , _ => None , } } fn is_flagged (& self , edge : G :: EdgeId) -> bool { matches ! (self , Label :: Flag (flag) if flag == & edge) } }
    };
}

impl_411!();