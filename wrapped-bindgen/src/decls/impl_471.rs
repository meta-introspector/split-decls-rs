macro_rules! deps {
    () => {
        AsRow!();
        HasAttributes!();
        Value!();
        GUID!();
        RowIterator!();
    };
}

macro_rules! impl_471 {
    () => {
        deps!();
        impl < R : AsRow + Into < HasAttribute > > HasAttributes for R { fn attributes (& self) -> RowIterator < Attribute > { self . file () . equal_range (0 , Into :: < HasAttribute > :: into (* self) . encode ()) } fn find_attribute (& self , name : & str) -> Option < Attribute > { self . attributes () . find (| attribute | attribute . name () == name) } fn has_attribute (& self , name : & str) -> bool { self . find_attribute (name) . is_some () } fn guid_attribute (& self) -> Option < GUID > { self . find_attribute ("GuidAttribute") . map (| attribute | { fn unwrap_u32 (value : & Value) -> u32 { match value { Value :: U32 (value) => * value , _ => panic ! () , } } fn unwrap_u16 (value : & Value) -> u16 { match value { Value :: U16 (value) => * value , rest => panic ! ("{rest:?}") , } } fn unwrap_u8 (value : & Value) -> u8 { match value { Value :: U8 (value) => * value , rest => panic ! ("{rest:?}") , } } let args = attribute . args () ; GUID (unwrap_u32 (& args [0] . 1) , unwrap_u16 (& args [1] . 1) , unwrap_u16 (& args [2] . 1) , unwrap_u8 (& args [3] . 1) , unwrap_u8 (& args [4] . 1) , unwrap_u8 (& args [5] . 1) , unwrap_u8 (& args [6] . 1) , unwrap_u8 (& args [7] . 1) , unwrap_u8 (& args [8] . 1) , unwrap_u8 (& args [9] . 1) , unwrap_u8 (& args [10] . 1) ,) }) } fn arches (& self) -> i32 { let mut arches = 0 ; if let Some (attribute) = self . find_attribute ("SupportedArchitectureAttribute") { if let Some ((_ , Value :: I32 (value))) = attribute . args () . first () { arches = * value ; } } arches } }
    };
}

impl_471!();