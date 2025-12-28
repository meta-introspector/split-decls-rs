macro_rules! deps {
    () => {
        Params!();
        ParamsIter!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a Params { type IntoIter = ParamsIter < 'a > ; type Item = & 'a [u16] ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_4!()