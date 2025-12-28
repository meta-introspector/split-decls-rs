macro_rules! deps {
    () => {
        SynInfo!();
    };
}

macro_rules! SynDetails {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub enum SynDetails { Info (SynInfo) , Error (String) , Unknown , }
    };
}

SynDetails!()