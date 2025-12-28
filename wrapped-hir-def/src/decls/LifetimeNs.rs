macro_rules! deps {
    () => {
        Static!();
        LifetimeParamId!();
    };
}

macro_rules! LifetimeNs {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum LifetimeNs { Static , LifetimeParam (LifetimeParamId) , }
    };
}

LifetimeNs!();