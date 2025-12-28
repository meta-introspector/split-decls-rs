macro_rules! deps {
    () => {
        Reader!();
        PubNamesEntry!();
        DebugInfoOffset!();
        UnitOffset!();
        PubStuffEntry!();
    };
}

macro_rules! impl_529 {
    () => {
        deps!();
        impl < R : Reader > PubStuffEntry < R > for PubNamesEntry < R > { fn new (die_offset : UnitOffset < R :: Offset > , name : R , unit_header_offset : DebugInfoOffset < R :: Offset > ,) -> Self { PubNamesEntry { unit_header_offset , die_offset , name , } } }
    };
}

impl_529!()