macro_rules! deps {
    () => {
        DisplayKind!();
    };
}

macro_rules! impl_487 {
    () => {
        deps!();
        impl DisplayKind { fn is_source_code (self) -> bool { matches ! (self , Self :: SourceCode { .. }) } fn allows_opaque (self) -> bool { match self { Self :: SourceCode { allow_opaque , .. } => allow_opaque , _ => true , } } }
    };
}

impl_487!()