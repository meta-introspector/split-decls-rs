macro_rules! deps {
    () => {
        EdgeType!();
        IndexType!();
        Edges!();
    };
}

macro_rules! impl_703 {
    () => {
        deps!();
        impl < E , Ty , Ix > Clone for Edges < '_ , E , Ty , Ix > where Ix : IndexType , Ty : EdgeType , { fn clone (& self) -> Self { Edges { skip_start : self . skip_start , edges : self . edges , next : self . next , direction : self . direction , ty : self . ty , } } }
    };
}

impl_703!();