macro_rules! deps {
    () => {
        CaptureUsageSource!();
    };
}

macro_rules! impl_390 {
    () => {
        deps!();
        impl CaptureUsageSource { pub fn source (& self) -> AstPtr < Either < ast :: Expr , ast :: Pat > > { self . source . value } pub fn file_id (& self) -> HirFileId { self . source . file_id } pub fn is_ref (& self) -> bool { self . is_ref } }
    };
}

impl_390!();