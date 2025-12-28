macro_rules! deps {
    () => {
        UpmappingResult!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < T > IntoIterator for UpmappingResult < T > { type Item = T ; type IntoIter = < ArrayVec < T , 2 > as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . def_site . into_iter () . chain (Some (self . call_site)) . collect :: < ArrayVec < _ , 2 > > () . into_iter () } }
    };
}

impl_56!();