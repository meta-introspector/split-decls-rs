macro_rules! deps {
    () => {
        UnitOffset!();
        Reader!();
        DebugInfoOffset!();
    };
}

macro_rules! PubStuffEntry {
    () => {
        deps!();
        pub trait PubStuffEntry < R : Reader > { fn new (die_offset : UnitOffset < R :: Offset > , name : R , unit_header_offset : DebugInfoOffset < R :: Offset > ,) -> Self ; }
    };
}

PubStuffEntry!()