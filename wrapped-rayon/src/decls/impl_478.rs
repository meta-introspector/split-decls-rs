macro_rules! deps {
    () => {
        Folder!();
        FilterMapFolder!();
    };
}

macro_rules! impl_478 {
    () => {
        deps!();
        impl < 'p , T , U , C , P > Folder < T > for FilterMapFolder < 'p , C , P > where C : Folder < U > , P : Fn (T) -> Option < U > + Sync + 'p , { type Result = C :: Result ; fn consume (self , item : T) -> Self { let filter_op = self . filter_op ; if let Some (mapped_item) = filter_op (item) { let base = self . base . consume (mapped_item) ; FilterMapFolder { base , filter_op } } else { self } } fn complete (self) -> C :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_478!();