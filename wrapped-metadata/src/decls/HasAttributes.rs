macro_rules! deps {
    () => {
        RowIterator!();
        Attribute!();
    };
}

macro_rules! HasAttributes {
    () => {
        deps!();
        pub trait HasAttributes < 'a > { fn attributes (& self) -> RowIterator < 'a , Attribute < 'a > > ; fn find_attribute (& self , name : & str) -> Option < Attribute < 'a > > ; fn has_attribute (& self , name : & str) -> bool ; }
    };
}

HasAttributes!();