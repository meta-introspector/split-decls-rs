macro_rules! deps {
    () => {
        DebugInfoRef!();
    };
}

macro_rules! Reference {
    () => {
        deps!();
        # [doc = " A reference to a `.debug_info` entry."] # [deprecated (note = "Renamed to DebugInfoRef")] pub type Reference = DebugInfoRef ;
    };
}

Reference!()