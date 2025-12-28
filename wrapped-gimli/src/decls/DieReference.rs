macro_rules! deps {
    () => {
        DebugInfoRef!();
        UnitRef!();
        UnitOffset!();
        DebugInfoOffset!();
    };
}

macro_rules! DieReference {
    () => {
        deps!();
        # [doc = " A reference to a DIE, either relative to the current CU or"] # [doc = " relative to the section."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum DieReference < T = usize > { # [doc = " A CU-relative reference."] UnitRef (UnitOffset < T >) , # [doc = " A section-relative reference."] DebugInfoRef (DebugInfoOffset < T >) , }
    };
}

DieReference!()