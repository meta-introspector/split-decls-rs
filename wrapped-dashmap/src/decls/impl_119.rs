macro_rules! deps {
    () => {
        DashSet!();
        OwningIter!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < K : Eq + Hash , S : BuildHasher + Clone > IntoIterator for DashSet < K , S > { type Item = K ; type IntoIter = OwningIter < K > ; fn into_iter (self) -> Self :: IntoIter { OwningIter :: new (self . inner . into_iter ()) } }
    };
}

impl_119!();