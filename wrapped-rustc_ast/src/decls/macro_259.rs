macro_rules! deps {
    () => {
        Ty!();
        Attribute!();
        Pat!();
        Block!();
        Path!();
        AttrItem!();
        Visibility!();
    };
}

macro_rules! macro_259 {
    () => {
        deps!();
        impl_has_attrs_none ! (Attribute , AttrItem , Block , Pat , Path , Ty , Visibility) ;
    };
}

macro_259!();