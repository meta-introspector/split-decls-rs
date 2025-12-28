macro_rules! deps {
    () => {
        AttrIdGenerator!();
        Attribute!();
        Path!();
        AttrArgs!();
        AttrItem!();
        AttrStyle!();
        Safety!();
    };
}

macro_rules! mk_attr {
    () => {
        deps!();
        fn mk_attr (g : & AttrIdGenerator , style : AttrStyle , unsafety : Safety , path : Path , args : AttrArgs , span : Span ,) -> Attribute { mk_attr_from_item (g , AttrItem { unsafety , path , args , tokens : None } , None , style , span) }
    };
}

mk_attr!();