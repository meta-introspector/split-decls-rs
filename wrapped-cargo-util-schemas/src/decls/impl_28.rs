macro_rules! deps {
    () => {
        SourceKind!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl SourceKind { pub fn protocol (& self) -> Option < & str > { match self { SourceKind :: Path => Some ("path") , SourceKind :: Git (_) => Some ("git") , SourceKind :: Registry => Some ("registry") , SourceKind :: SparseRegistry => None , SourceKind :: LocalRegistry => Some ("local-registry") , SourceKind :: Directory => Some ("directory") , } } }
    };
}

impl_28!();