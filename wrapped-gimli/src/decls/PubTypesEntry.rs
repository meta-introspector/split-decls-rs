macro_rules! deps {
    () => {
        UnitOffset!();
        DebugInfoOffset!();
        Reader!();
    };
}

macro_rules! PubTypesEntry {
    () => {
        deps!();
        # [doc = " A single parsed pubtype."] # [derive (Debug , Clone)] pub struct PubTypesEntry < R : Reader > { unit_header_offset : DebugInfoOffset < R :: Offset > , die_offset : UnitOffset < R :: Offset > , name : R , }
    };
}

PubTypesEntry!()