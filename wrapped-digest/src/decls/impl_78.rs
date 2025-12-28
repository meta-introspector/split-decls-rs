macro_rules! deps {
    () => {
        CollisionResistance!();
        ExtendableOutput!();
        XofFixedWrapper!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < T : ExtendableOutput + CollisionResistance , S : ArraySize > CollisionResistance for XofFixedWrapper < T , S > { type CollisionResistance = T :: CollisionResistance ; }
    };
}

impl_78!();