macro_rules! deps {
    () => {
        Reader!();
        UnitOffset!();
        DebugInfoOffset!();
    };
}

macro_rules! PubNamesEntry {
    () => {
        deps!();
        # [doc = " A single parsed pubname."] # [derive (Debug , Clone)] pub struct PubNamesEntry < R : Reader > { unit_header_offset : DebugInfoOffset < R :: Offset > , die_offset : UnitOffset < R :: Offset > , name : R , }
    };
}

PubNamesEntry!()