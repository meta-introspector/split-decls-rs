macro_rules! deps {
    () => {
        HistoryBufInner!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < T , S : HistoryBufStorage < T > + ? Sized > Deref for HistoryBufInner < T , S > { type Target = [T] ; fn deref (& self) -> & [T] { self . as_slice () } }
    };
}

impl_68!()