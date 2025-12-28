macro_rules! deps {
    () => {
        NodeKind!();
        SsrError!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl NodeKind { fn from (name : & SmolStr) -> Result < NodeKind , SsrError > { Ok (match name . as_str () { "literal" => NodeKind :: Literal , _ => bail ! ("Unknown node kind '{}'" , name) , }) } }
    };
}

impl_61!();