macro_rules! deps {
    () => {
        MapWithFolder!();
        Folder!();
    };
}

macro_rules! impl_704 {
    () => {
        deps!();
        impl < 'f , T , U , R , C , F > Folder < T > for MapWithFolder < 'f , C , U , F > where C : Folder < R > , F : Fn (& mut U , T) -> R , { type Result = C :: Result ; fn consume (mut self , item : T) -> Self { let mapped_item = (self . map_op) (& mut self . item , item) ; self . base = self . base . consume (mapped_item) ; self } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = T > , { fn with < 'f , T , U , R > (item : & 'f mut U , map_op : impl Fn (& mut U , T) -> R + 'f ,) -> impl FnMut (T) -> R + 'f { move | x | map_op (item , x) } { let mapped_iter = iter . into_iter () . map (with (& mut self . item , self . map_op)) ; self . base = self . base . consume_iter (mapped_iter) ; } self } fn complete (self) -> C :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_704!()