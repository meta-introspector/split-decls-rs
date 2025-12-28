macro_rules! deps {
    () => {
        LazyAttrTokenStream!();
        AttrItem!();
        Walkable!();
    };
}

macro_rules! NormalAttr {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct NormalAttr { pub item : AttrItem , pub tokens : Option < LazyAttrTokenStream > , }
    };
}

NormalAttr!();