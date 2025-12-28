macro_rules! deps {
    () => {
        RustcToolInfo!();
    };
}

macro_rules! RustcToolDetails {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub enum RustcToolDetails { Info (RustcToolInfo) , Error (String) , Unknown , }
    };
}

RustcToolDetails!()