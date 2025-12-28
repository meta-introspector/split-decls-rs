macro_rules! deps {
    () => {
        LlvmInfo!();
    };
}

macro_rules! LlvmDetails {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub enum LlvmDetails { Info (LlvmInfo) , Error (String) , Unknown , }
    };
}

LlvmDetails!();