macro_rules! deps {
    () => {
        WhileSomeFolder!();
        Folder!();
    };
}

macro_rules! impl_1010 {
    () => {
        deps!();
        impl < 'f , T , C > Folder < Option < T > > for WhileSomeFolder < 'f , C > where C : Folder < T > , { type Result = C :: Result ; fn consume (mut self , item : Option < T >) -> Self { match item { Some (item) => self . base = self . base . consume (item) , None => self . full . store (true , Ordering :: Relaxed) , } self } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = Option < T > > , { fn some < T > (full : & AtomicBool) -> impl Fn (& Option < T >) -> bool + '_ { move | x | match * x { Some (_) => ! full . load (Ordering :: Relaxed) , None => { full . store (true , Ordering :: Relaxed) ; false } } } self . base = self . base . consume_iter (iter . into_iter () . take_while (some (self . full)) . map (Option :: unwrap) ,) ; self } fn complete (self) -> C :: Result { self . base . complete () } fn full (& self) -> bool { self . full . load (Ordering :: Relaxed) || self . base . full () } }
    };
}

impl_1010!()