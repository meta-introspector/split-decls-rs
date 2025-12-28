macro_rules! LifetimeParam {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct LifetimeParam { pub (crate) id : LifetimeParamId , }
    };
}

LifetimeParam!()