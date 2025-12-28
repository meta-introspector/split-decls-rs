macro_rules! deps {
    () => {
        Utf8Parser!();
    };
}

macro_rules! StripBytes {
    () => {
        deps!();
        # [doc = " Incrementally strip non-contiguous data"] # [derive (Default , Clone , Debug , PartialEq , Eq)] pub struct StripBytes { state : State , utf8parser : Utf8Parser , }
    };
}

StripBytes!()