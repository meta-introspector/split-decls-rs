macro_rules! deps {
    () => {
        NixInfo!();
    };
}

macro_rules! NixDetails {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub enum NixDetails { Info (NixInfo) , Error (String) , Unknown , }
    };
}

NixDetails!()