macro_rules! deps {
    () => {
        CtorKind!();
        VariantData!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl CtorKind { pub fn from_ast (vdata : & ast :: VariantData) -> Option < (CtorKind , NodeId) > { match * vdata { ast :: VariantData :: Tuple (_ , node_id) => Some ((CtorKind :: Fn , node_id)) , ast :: VariantData :: Unit (node_id) => Some ((CtorKind :: Const , node_id)) , ast :: VariantData :: Struct { .. } => None , } } }
    };
}

impl_73!()