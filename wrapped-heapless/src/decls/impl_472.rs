macro_rules! deps {
    () => {
        Storage!();
        IntoIter!();
        Iter!();
        QueueInner!();
    };
}

macro_rules! impl_472 {
    () => {
        deps!();
        impl < 'a , T , S : Storage > IntoIterator for & 'a QueueInner < T , S > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_472!()