macro_rules! deps {
    () => {
        MapFolder!();
        Folder!();
    };
}

macro_rules! impl_686 {
    () => {
        deps!();
        impl < 'f , T , R , C , F > Folder < T > for MapFolder < 'f , C , F > where C : Folder < F :: Output > , F : Fn (T) -> R , { type Result = C :: Result ; fn consume (self , item : T) -> Self { let mapped_item = (self . map_op) (item) ; MapFolder { base : self . base . consume (mapped_item) , map_op : self . map_op , } } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = T > , { self . base = self . base . consume_iter (iter . into_iter () . map (self . map_op)) ; self } fn complete (self) -> C :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_686!()