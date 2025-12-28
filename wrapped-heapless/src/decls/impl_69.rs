macro_rules! deps {
    () => {
        HistoryBufInner!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < T , S : HistoryBufStorage < T > + ? Sized > AsRef < [T] > for HistoryBufInner < T , S > { # [inline] fn as_ref (& self) -> & [T] { self } }
    };
}

impl_69!()