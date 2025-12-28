macro_rules! deps {
    () => {
        SetMatches!();
        SetMatchesIter!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a SetMatches { type IntoIter = SetMatchesIter < 'a > ; type Item = usize ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_142!();