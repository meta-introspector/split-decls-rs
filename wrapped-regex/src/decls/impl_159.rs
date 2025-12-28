macro_rules! deps {
    () => {
        SetMatchesIter!();
        SetMatches!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a SetMatches { type IntoIter = SetMatchesIter < 'a > ; type Item = usize ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_159!()