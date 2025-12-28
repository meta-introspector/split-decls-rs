macro_rules! deps {
    () => {
        StatusIter!();
        Statuses!();
        StatusEntry!();
    };
}

macro_rules! impl_758 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a Statuses < 'a > { type Item = StatusEntry < 'a > ; type IntoIter = StatusIter < 'a > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_758!()