macro_rules! deps {
    () => {
        Load!();
        Spurious!();
        Schedule!();
    };
}

macro_rules! macro_120 {
    () => {
        deps!();
        objects ! { # [derive (Debug)] # [cfg_attr (feature = "checkpoint" , derive (Serialize , Deserialize))] Entry , Schedule (Schedule) , Load (Load) , Spurious (Spurious) , }
    };
}

macro_120!()