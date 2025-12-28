macro_rules! deps {
    () => {
        PathResolution!();
    };
}

macro_rules! PathResolutionPerNs {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub struct PathResolutionPerNs { pub type_ns : Option < PathResolution > , pub value_ns : Option < PathResolution > , pub macro_ns : Option < PathResolution > , }
    };
}

PathResolutionPerNs!()