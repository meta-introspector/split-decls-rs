macro_rules! deps {
    () => {
        PubTypesEntry!();
        DebugInfoOffset!();
        UnitOffset!();
        Reader!();
        PubStuffEntry!();
    };
}

macro_rules! impl_541 {
    () => {
        deps!();
        impl < R : Reader > PubStuffEntry < R > for PubTypesEntry < R > { fn new (die_offset : UnitOffset < R :: Offset > , name : R , unit_header_offset : DebugInfoOffset < R :: Offset > ,) -> Self { PubTypesEntry { unit_header_offset , die_offset , name , } } }
    };
}

impl_541!();