macro_rules! deps {
    () => {
        Folder!();
        ClonedFolder!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl < 'a , T , F > Folder < & 'a T > for ClonedFolder < F > where F : Folder < T > , T : 'a + Clone , { type Result = F :: Result ; fn consume (self , item : & 'a T) -> Self { ClonedFolder { base : self . base . consume (item . clone ()) , } } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = & 'a T > , { self . base = self . base . consume_iter (iter . into_iter () . cloned ()) ; self } fn complete (self) -> F :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_338!();