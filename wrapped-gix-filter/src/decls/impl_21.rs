macro_rules! deps {
    () => {
        AttributesDigest!();
        Mode!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl From < Mode > for AttributesDigest { fn from (value : Mode) -> Self { match value { Mode :: Lf => AttributesDigest :: TextInput , Mode :: CrLf => AttributesDigest :: TextCrlf , } } }
    };
}

impl_21!();