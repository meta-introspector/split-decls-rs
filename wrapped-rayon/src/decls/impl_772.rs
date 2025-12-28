macro_rules! deps {
    () => {
        Folder!();
        PositionsFolder!();
    };
}

macro_rules! impl_772 {
    () => {
        deps!();
        impl < F , P , T > Folder < T > for PositionsFolder < '_ , F , P > where F : Folder < usize > , P : Fn (T) -> bool , { type Result = F :: Result ; fn consume (mut self , item : T) -> Self { let index = self . offset ; self . offset += 1 ; if (self . predicate) (item) { self . base = self . base . consume (index) ; } self } fn complete (self) -> Self :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_772!()