macro_rules! deps {
    () => {
        ConstParam!();
        LifetimeParam!();
        TypeParam!();
    };
}

macro_rules! GenericParam {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum GenericParam { TypeParam (TypeParam) , ConstParam (ConstParam) , LifetimeParam (LifetimeParam) , }
    };
}

GenericParam!()