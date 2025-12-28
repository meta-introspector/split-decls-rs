macro_rules! deps {
    () => {
        EdgeSet!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < A > AddAssign for EdgeSet < A > { fn add_assign (& mut self , mut rhs : Self) { self . set . append (& mut rhs . set) } }
    };
}

impl_103!()