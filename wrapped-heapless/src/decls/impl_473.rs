macro_rules! deps {
    () => {
        QueueInner!();
        IterMut!();
        IntoIter!();
        Storage!();
    };
}

macro_rules! impl_473 {
    () => {
        deps!();
        impl < 'a , T , S : Storage > IntoIterator for & 'a mut QueueInner < T , S > { type Item = & 'a mut T ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
    };
}

impl_473!()