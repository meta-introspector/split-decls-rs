macro_rules! deps {
    () => {
        TakeAnyWhileFolder!();
        Folder!();
    };
}

macro_rules! impl_895 {
    () => {
        deps!();
        impl < 'p , T , C , P > Folder < T > for TakeAnyWhileFolder < 'p , C , P > where C : Folder < T > , P : Fn (& T) -> bool + 'p , { type Result = C :: Result ; fn consume (mut self , item : T) -> Self { if take (& item , self . taking , self . predicate) { self . base = self . base . consume (item) ; } self } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = T > , { self . base = self . base . consume_iter (iter . into_iter () . take_while (move | x | take (x , self . taking , self . predicate)) ,) ; self } fn complete (self) -> C :: Result { self . base . complete () } fn full (& self) -> bool { ! self . taking . load (Ordering :: Relaxed) || self . base . full () } }
    };
}

impl_895!();