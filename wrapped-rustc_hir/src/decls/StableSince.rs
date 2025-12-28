macro_rules! deps {
    () => {
        RustcVersion!();
        PrintAttribute!();
    };
}

macro_rules! StableSince {
    () => {
        deps!();
        # [doc = " Rust release in which a feature is stabilized."] # [derive (Encodable , Decodable , PartialEq , Copy , Clone , Debug , Eq , PartialOrd , Ord , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub enum StableSince { # [doc = " also stores the original symbol for printing"] Version (RustcVersion) , # [doc = " Stabilized in the upcoming version, whatever number that is."] Current , # [doc = " Failed to parse a stabilization version."] Err (ErrorGuaranteed) , }
    };
}

StableSince!();