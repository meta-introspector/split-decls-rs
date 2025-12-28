macro_rules! deps {
    () => {
        Walkable!();
        Path!();
    };
}

macro_rules! VisibilityKind {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum VisibilityKind { Public , Restricted { path : Box < Path > , id : NodeId , shorthand : bool } , Inherited , }
    };
}

VisibilityKind!();