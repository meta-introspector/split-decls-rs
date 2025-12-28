macro_rules! deps {
    () => {
        UpdateFolder!();
        Folder!();
    };
}

macro_rules! impl_981 {
    () => {
        deps!();
        impl < 'f , T , C , F > Folder < T > for UpdateFolder < 'f , C , F > where C : Folder < T > , F : Fn (& mut T) , { type Result = C :: Result ; fn consume (self , mut item : T) -> Self { (self . update_op) (& mut item) ; UpdateFolder { base : self . base . consume (item) , update_op : self . update_op , } } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = T > , { let update_op = self . update_op ; self . base = self . base . consume_iter (iter . into_iter () . map (apply (update_op))) ; self } fn complete (self) -> C :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_981!();