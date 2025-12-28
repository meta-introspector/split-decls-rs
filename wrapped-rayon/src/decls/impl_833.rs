macro_rules! deps {
    () => {
        SkipAnyFolder!();
        Folder!();
    };
}

macro_rules! impl_833 {
    () => {
        deps!();
        impl < 'f , T , C > Folder < T > for SkipAnyFolder < 'f , C > where C : Folder < T > , { type Result = C :: Result ; fn consume (mut self , item : T) -> Self { if ! checked_decrement (self . count) { self . base = self . base . consume (item) ; } self } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = T > , { self . base = self . base . consume_iter (iter . into_iter () . skip_while (move | _ | checked_decrement (self . count)) ,) ; self } fn complete (self) -> C :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_833!()