macro_rules! deps {
    () => {
        LifetimeParam!();
        TypeParam!();
        ConstParam!();
    };
}

macro_rules! GenericParam {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum GenericParam { TypeParam (TypeParam) , ConstParam (ConstParam) , LifetimeParam (LifetimeParam) , }
    };
}

GenericParam!();