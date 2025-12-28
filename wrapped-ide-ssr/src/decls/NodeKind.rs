macro_rules! NodeKind {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) enum NodeKind { Literal , }
    };
}

NodeKind!()