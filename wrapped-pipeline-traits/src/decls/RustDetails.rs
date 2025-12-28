macro_rules! deps {
    () => {
        RustDetailsInfo!();
    };
}

macro_rules! RustDetails {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub enum RustDetails { Info (RustDetailsInfo) , Error (String) , Unknown , }
    };
}

RustDetails!();