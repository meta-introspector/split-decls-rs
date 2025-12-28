macro_rules! deps {
    () => {
        FlatSet!();
    };
}

macro_rules! impl_633 {
    () => {
        deps!();
        impl < T : PartialEq + Eq > IntoIterator for FlatSet < T > { type Item = T ; type IntoIter = std :: vec :: IntoIter < T > ; fn into_iter (self) -> Self :: IntoIter { self . inner . into_iter () } }
    };
}

impl_633!();