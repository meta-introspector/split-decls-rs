macro_rules! deps {
    () => {
        LazyAttrTokenStream!();
        Walkable!();
        Path!();
        AttrArgs!();
        Safety!();
    };
}

macro_rules! AttrItem {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct AttrItem { pub unsafety : Safety , pub path : Path , pub args : AttrArgs , pub tokens : Option < LazyAttrTokenStream > , }
    };
}

AttrItem!()