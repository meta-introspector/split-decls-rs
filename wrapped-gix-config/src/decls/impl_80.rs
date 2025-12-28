macro_rules! deps {
    () => {
        BodyIter!();
        Body!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < 'event > IntoIterator for Body < 'event > { type Item = (ValueName < 'event > , Cow < 'event , BStr >) ; type IntoIter = BodyIter < 'event > ; fn into_iter (self) -> Self :: IntoIter { BodyIter (self . 0 . into_iter ()) } }
    };
}

impl_80!()