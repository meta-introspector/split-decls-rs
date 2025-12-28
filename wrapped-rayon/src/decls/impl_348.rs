macro_rules! deps {
    () => {
        Folder!();
        CollectResult!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl < 'c , T : Send + 'c > Folder < T > for CollectResult < 'c , T > { type Result = Self ; fn consume (mut self , item : T) -> Self { assert ! (self . initialized_len < self . total_len , "too many values pushed to consumer") ; unsafe { self . start . 0 . add (self . initialized_len) . write (item) ; self . initialized_len += 1 ; } self } fn complete (self) -> Self :: Result { self } fn full (& self) -> bool { false } }
    };
}

impl_348!()