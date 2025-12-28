macro_rules! deps {
    () => {
        Idx!();
        Arena!();
        IntoIter!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T > IntoIterator for Arena < T > { type Item = (Idx < T > , T) ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> Self :: IntoIter { IntoIter (self . data . into_iter () . enumerate ()) } }
    };
}

impl_38!()