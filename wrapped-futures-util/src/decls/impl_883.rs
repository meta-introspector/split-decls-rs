macro_rules! deps {
    () => {
        IntoIter!();
        FuturesUnordered!();
    };
}

macro_rules! impl_883 {
    () => {
        deps!();
        impl < Fut : Unpin > IntoIterator for FuturesUnordered < Fut > { type Item = Fut ; type IntoIter = IntoIter < Fut > ; fn into_iter (mut self) -> Self :: IntoIter { let task = * self . head_all . get_mut () ; let len = if task . is_null () { 0 } else { unsafe { * (* task) . len_all . get () } } ; IntoIter { len , inner : self } } }
    };
}

impl_883!()