macro_rules! deps {
    () => {
        AttrArgs!();
        Safety!();
        LazyAttrTokenStream!();
        Path!();
        Walkable!();
    };
}

macro_rules! AttrItem {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct AttrItem { pub unsafety : Safety , pub path : Path , pub args : AttrArgs , pub tokens : Option < LazyAttrTokenStream > , }
    };
}

AttrItem!();