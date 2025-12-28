macro_rules! deps {
    () => {
        Attribute!();
        AsRow!();
        HasAttributes!();
        RowIterator!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < 'a , R : AsRow < 'a > + Into < HasAttribute < 'a > > > HasAttributes < 'a > for R { fn attributes (& self) -> RowIterator < 'a , Attribute < 'a > > { self . equal_range (0 , Into :: < HasAttribute > :: into (* self) . encode ()) } fn find_attribute (& self , name : & str) -> Option < Attribute < 'a > > { self . attributes () . find (| attribute | attribute . ctor () . parent () . name () == name) } fn has_attribute (& self , name : & str) -> bool { self . find_attribute (name) . is_some () } }
    };
}

impl_75!();