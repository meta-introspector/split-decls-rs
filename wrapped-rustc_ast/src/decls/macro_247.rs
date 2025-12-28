macro_rules! deps {
    () => {
        Path!();
        AttrItem!();
        Expr!();
        Item!();
        Block!();
        Visibility!();
        Ty!();
        Pat!();
        AssocItem!();
        ForeignItem!();
    };
}

macro_rules! macro_247 {
    () => {
        deps!();
        impl_has_tokens ! (AssocItem , AttrItem , Block , Expr , ForeignItem , Item , Pat , Path , Ty , Visibility) ;
    };
}

macro_247!()