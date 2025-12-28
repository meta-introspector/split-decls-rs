macro_rules! deps {
    () => {
        RowIterator!();
        GUID!();
    };
}

macro_rules! HasAttributes {
    () => {
        deps!();
        pub trait HasAttributes { fn attributes (& self) -> RowIterator < Attribute > ; fn find_attribute (& self , name : & str) -> Option < Attribute > ; fn has_attribute (& self , name : & str) -> bool ; fn guid_attribute (& self) -> Option < GUID > ; fn arches (& self) -> i32 ; }
    };
}

HasAttributes!();