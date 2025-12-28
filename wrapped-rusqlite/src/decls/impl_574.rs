macro_rules! deps {
    () => {
        ValueIter!();
        Values!();
        ValueRef!();
    };
}

macro_rules! impl_574 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a Values < 'a > { type IntoIter = ValueIter < 'a > ; type Item = ValueRef < 'a > ; # [inline] fn into_iter (self) -> ValueIter < 'a > { self . iter () } }
    };
}

impl_574!()