macro_rules! deps {
    () => {
        InspectFolder!();
        Folder!();
    };
}

macro_rules! impl_624 {
    () => {
        deps!();
        impl < 'f , T , C , F > Folder < T > for InspectFolder < 'f , C , F > where C : Folder < T > , F : Fn (& T) , { type Result = C :: Result ; fn consume (self , item : T) -> Self { (self . inspect_op) (& item) ; InspectFolder { base : self . base . consume (item) , inspect_op : self . inspect_op , } } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = T > , { self . base = self . base . consume_iter (iter . into_iter () . inspect (self . inspect_op)) ; self } fn complete (self) -> C :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_624!();