macro_rules! deps {
    () => {
        Endian!();
        U32!();
    };
}

macro_rules! BuildToolVersion {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct BuildToolVersion < E : Endian > { # [doc = " enum for the tool"] pub tool : U32 < E > , # [doc = " version number of the tool"] pub version : U32 < E > , }
    };
}

BuildToolVersion!()