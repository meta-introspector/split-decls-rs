macro_rules! deps {
    () => {
        SetMatchesIntoIter!();
        SetMatches!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl IntoIterator for SetMatches { type IntoIter = SetMatchesIntoIter ; type Item = usize ; fn into_iter (self) -> Self :: IntoIter { let it = 0 .. self . 0 . capacity () ; SetMatchesIntoIter { patset : self . 0 , it } } }
    };
}

impl_141!()