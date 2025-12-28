macro_rules! deps {
    () => {
        HistoryBuf!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < T , const N : usize > Clone for HistoryBuf < T , N > where T : Clone , { fn clone (& self) -> Self { let mut ret = Self :: new () ; for (new , old) in ret . data . borrow_mut () . iter_mut () . zip (self . as_slice ()) { new . write (old . clone ()) ; } ret . filled = self . filled ; ret . write_at = self . write_at ; ret } }
    };
}

impl_66!()