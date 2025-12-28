macro_rules! deps {
    () => {
        Attribute!();
        Pat!();
        Ty!();
        AttrItem!();
        Block!();
        Path!();
        Visibility!();
    };
}

macro_rules! macro_259 {
    () => {
        deps!();
        impl_has_attrs_none ! (Attribute , AttrItem , Block , Pat , Path , Ty , Visibility) ;
    };
}

macro_259!()