macro_rules! Version {
    () => {
        # [doc = " TZif version"] # [derive (Debug , Copy , Clone , Eq , PartialEq)] enum Version { # [doc = " Version 1"] V1 , # [doc = " Version 2"] V2 , # [doc = " Version 3"] V3 , }
    };
}

Version!()