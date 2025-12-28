macro_rules! deps {
    () => {
        CopiedFolder!();
        Folder!();
    };
}

macro_rules! impl_390 {
    () => {
        deps!();
        impl < 'a , T , F > Folder < & 'a T > for CopiedFolder < F > where F : Folder < T > , T : 'a + Copy , { type Result = F :: Result ; fn consume (self , & item : & 'a T) -> Self { CopiedFolder { base : self . base . consume (item) , } } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = & 'a T > , { self . base = self . base . consume_iter (iter . into_iter () . copied ()) ; self } fn complete (self) -> F :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_390!();