macro_rules! deps {
    () => {
        TextEdit!();
        Indel!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl IntoIterator for TextEdit { type Item = Indel ; type IntoIter = std :: vec :: IntoIter < Indel > ; fn into_iter (self) -> Self :: IntoIter { self . indels . into_iter () } }
    };
}

impl_220!()