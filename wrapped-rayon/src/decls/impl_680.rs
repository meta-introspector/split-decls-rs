macro_rules! deps {
    () => {
        MapProducer!();
        Map!();
        Producer!();
        Folder!();
        MapFolder!();
        IntoIter!();
    };
}

macro_rules! impl_680 {
    () => {
        deps!();
        impl < 'f , P , F , R > Producer for MapProducer < 'f , P , F > where P : Producer , F : Fn (P :: Item) -> R + Sync , R : Send , { type Item = F :: Output ; type IntoIter = iter :: Map < P :: IntoIter , & 'f F > ; fn into_iter (self) -> Self :: IntoIter { self . base . into_iter () . map (self . map_op) } fn min_len (& self) -> usize { self . base . min_len () } fn max_len (& self) -> usize { self . base . max_len () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . base . split_at (index) ; (MapProducer { base : left , map_op : self . map_op , } , MapProducer { base : right , map_op : self . map_op , } ,) } fn fold_with < G > (self , folder : G) -> G where G : Folder < Self :: Item > , { let folder1 = MapFolder { base : folder , map_op : self . map_op , } ; self . base . fold_with (folder1) . base } }
    };
}

impl_680!();