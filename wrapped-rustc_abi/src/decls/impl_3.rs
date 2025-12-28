macro_rules! deps {
    () => {
        HomogeneousAggregate!();
        Heterogeneous!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl HomogeneousAggregate { # [doc = " If this is a homogeneous aggregate, returns the homogeneous"] # [doc = " unit, else `None`."] pub fn unit (self) -> Option < Reg > { match self { HomogeneousAggregate :: Homogeneous (reg) => Some (reg) , HomogeneousAggregate :: NoData => None , } } # [doc = " Try to combine two `HomogeneousAggregate`s, e.g. from two fields in"] # [doc = " the same `struct`. Only succeeds if only one of them has any data,"] # [doc = " or both units are identical."] fn merge (self , other : HomogeneousAggregate) -> Result < HomogeneousAggregate , Heterogeneous > { match (self , other) { (x , HomogeneousAggregate :: NoData) | (HomogeneousAggregate :: NoData , x) => Ok (x) , (HomogeneousAggregate :: Homogeneous (a) , HomogeneousAggregate :: Homogeneous (b)) => { if a != b { return Err (Heterogeneous) ; } Ok (self) } } } }
    };
}

impl_3!()