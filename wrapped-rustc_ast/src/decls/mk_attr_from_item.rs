macro_rules! deps {
    () => {
        Attribute!();
        AttrKind!();
        LazyAttrTokenStream!();
        AttrStyle!();
        NormalAttr!();
        AttrIdGenerator!();
        AttrItem!();
    };
}

macro_rules! mk_attr_from_item {
    () => {
        deps!();
        pub fn mk_attr_from_item (g : & AttrIdGenerator , item : AttrItem , tokens : Option < LazyAttrTokenStream > , style : AttrStyle , span : Span ,) -> Attribute { Attribute { kind : AttrKind :: Normal (Box :: new (NormalAttr { item , tokens })) , id : g . mk_attr_id () , style , span , } }
    };
}

mk_attr_from_item!()