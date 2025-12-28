macro_rules! deps {
    () => {
        HistoryBufInner!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < T , S : HistoryBufStorage < T > + ? Sized > Drop for HistoryBufInner < T , S > { fn drop (& mut self) { unsafe { self . drop_contents () } } }
    };
}

impl_67!();