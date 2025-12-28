macro_rules! deps {
    () => {
        Bytes!();
        IntoIter!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl IntoIterator for Bytes { type Item = u8 ; type IntoIter = IntoIter < Bytes > ; fn into_iter (self) -> Self :: IntoIter { IntoIter :: new (self) } }
    };
}

impl_77!()