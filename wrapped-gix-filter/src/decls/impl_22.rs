macro_rules! deps {
    () => {
        AttributesDigest!();
        AutoCrlf!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl From < AutoCrlf > for AttributesDigest { fn from (value : AutoCrlf) -> Self { match value { AutoCrlf :: Input => AttributesDigest :: TextAutoInput , AutoCrlf :: Enabled => AttributesDigest :: TextAutoCrlf , AutoCrlf :: Disabled => AttributesDigest :: Binary , } } }
    };
}

impl_22!();