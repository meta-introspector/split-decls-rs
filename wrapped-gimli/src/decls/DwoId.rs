macro_rules! DwoId {
    () => {
        # [doc = " An optionally-provided implementation-defined compilation unit ID to enable"] # [doc = " split DWARF and linking a split compilation unit back together."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct DwoId (pub u64) ;
    };
}

DwoId!()