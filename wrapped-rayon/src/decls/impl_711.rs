macro_rules! deps {
    () => {
        IntoIter!();
        Folder!();
        MapWithIter!();
        MapInitProducer!();
        MapWithFolder!();
        Producer!();
    };
}

macro_rules! impl_711 {
    () => {
        deps!();
        impl < 'f , P , INIT , U , F , R > Producer for MapInitProducer < 'f , P , INIT , F > where P : Producer , INIT : Fn () -> U + Sync , F : Fn (& mut U , P :: Item) -> R + Sync , R : Send , { type Item = R ; type IntoIter = MapWithIter < 'f , P :: IntoIter , U , F > ; fn into_iter (self) -> Self :: IntoIter { MapWithIter { base : self . base . into_iter () , item : (self . init) () , map_op : self . map_op , } } fn min_len (& self) -> usize { self . base . min_len () } fn max_len (& self) -> usize { self . base . max_len () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . base . split_at (index) ; (MapInitProducer { base : left , init : self . init , map_op : self . map_op , } , MapInitProducer { base : right , init : self . init , map_op : self . map_op , } ,) } fn fold_with < G > (self , folder : G) -> G where G : Folder < Self :: Item > , { let folder1 = MapWithFolder { base : folder , item : (self . init) () , map_op : self . map_op , } ; self . base . fold_with (folder1) . base } }
    };
}

impl_711!();