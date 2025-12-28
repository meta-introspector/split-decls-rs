macro_rules! deps {
    () => {
        IntoIter!();
        Producer!();
        Folder!();
        MapWithIter!();
        MapWithFolder!();
        MapWithProducer!();
    };
}

macro_rules! impl_694 {
    () => {
        deps!();
        impl < 'f , P , U , F , R > Producer for MapWithProducer < 'f , P , U , F > where P : Producer , U : Send + Clone , F : Fn (& mut U , P :: Item) -> R + Sync , R : Send , { type Item = R ; type IntoIter = MapWithIter < 'f , P :: IntoIter , U , F > ; fn into_iter (self) -> Self :: IntoIter { MapWithIter { base : self . base . into_iter () , item : self . item , map_op : self . map_op , } } fn min_len (& self) -> usize { self . base . min_len () } fn max_len (& self) -> usize { self . base . max_len () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . base . split_at (index) ; (MapWithProducer { base : left , item : self . item . clone () , map_op : self . map_op , } , MapWithProducer { base : right , item : self . item , map_op : self . map_op , } ,) } fn fold_with < G > (self , folder : G) -> G where G : Folder < Self :: Item > , { let folder1 = MapWithFolder { base : folder , item : self . item , map_op : self . map_op , } ; self . base . fold_with (folder1) . base } }
    };
}

impl_694!()