macro_rules! deps {
    () => {
        Block!();
        Pat!();
        Expr!();
        AttrItem!();
        Ty!();
        Path!();
        AssocItem!();
        ForeignItem!();
        Item!();
        Visibility!();
    };
}

macro_rules! macro_247 {
    () => {
        deps!();
        impl_has_tokens ! (AssocItem , AttrItem , Block , Expr , ForeignItem , Item , Pat , Path , Ty , Visibility) ;
    };
}

macro_247!();