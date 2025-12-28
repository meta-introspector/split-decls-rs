macro_rules! deps {
    () => {
        UnitOffset!();
        DebugInfoOffset!();
        Reader!();
    };
}

macro_rules! PubStuffEntry {
    () => {
        deps!();
        pub trait PubStuffEntry < R : Reader > { fn new (die_offset : UnitOffset < R :: Offset > , name : R , unit_header_offset : DebugInfoOffset < R :: Offset > ,) -> Self ; }
    };
}

PubStuffEntry!();