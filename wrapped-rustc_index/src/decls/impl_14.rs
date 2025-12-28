macro_rules! deps {
    () => {
        Idx!();
        BitRelations!();
        DenseBitSet!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T : Idx > BitRelations < DenseBitSet < T > > for DenseBitSet < T > { fn union (& mut self , other : & DenseBitSet < T >) -> bool { assert_eq ! (self . domain_size , other . domain_size) ; bitwise (& mut self . words , & other . words , | a , b | a | b) } fn subtract (& mut self , other : & DenseBitSet < T >) -> bool { assert_eq ! (self . domain_size , other . domain_size) ; bitwise (& mut self . words , & other . words , | a , b | a & ! b) } fn intersect (& mut self , other : & DenseBitSet < T >) -> bool { assert_eq ! (self . domain_size , other . domain_size) ; bitwise (& mut self . words , & other . words , | a , b | a & b) } }
    };
}

impl_14!()