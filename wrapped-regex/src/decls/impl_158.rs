macro_rules! deps {
    () => {
        SetMatches!();
        SetMatchesIntoIter!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl IntoIterator for SetMatches { type IntoIter = SetMatchesIntoIter ; type Item = usize ; fn into_iter (self) -> Self :: IntoIter { let it = 0 .. self . 0 . capacity () ; SetMatchesIntoIter { patset : self . 0 , it } } }
    };
}

impl_158!();