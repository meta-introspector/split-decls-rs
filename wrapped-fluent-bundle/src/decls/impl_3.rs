macro_rules! deps {
    () => {
        FluentValue!();
        FluentArgs!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < 'args > IntoIterator for FluentArgs < 'args > { type Item = (Cow < 'args , str > , FluentValue < 'args >) ; type IntoIter = std :: vec :: IntoIter < Self :: Item > ; fn into_iter (self) -> Self :: IntoIter { self . 0 . into_iter () } }
    };
}

impl_3!();