macro_rules! deps {
    () => {
        HistoryBufInner!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < T , S : HistoryBufStorage < T > + ? Sized > PartialEq for HistoryBufInner < T , S > where T : PartialEq , { fn eq (& self , other : & Self) -> bool { self . oldest_ordered () . eq (other . oldest_ordered ()) } }
    };
}

impl_72!();