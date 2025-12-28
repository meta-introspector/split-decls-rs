macro_rules! deps {
    () => {
        Folder!();
        FilterFolder!();
    };
}

macro_rules! impl_467 {
    () => {
        deps!();
        impl < 'p , C , P , T > Folder < T > for FilterFolder < 'p , C , P > where C : Folder < T > , P : Fn (& T) -> bool + 'p , { type Result = C :: Result ; fn consume (self , item : T) -> Self { let filter_op = self . filter_op ; if filter_op (& item) { let base = self . base . consume (item) ; FilterFolder { base , filter_op } } else { self } } fn complete (self) -> Self :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_467!();