macro_rules! deps {
    () => {
        Folder!();
        Reducer!();
        MapWithFolder!();
        Consumer!();
        MapWithConsumer!();
    };
}

macro_rules! impl_701 {
    () => {
        deps!();
        impl < 'f , T , U , R , C , F > Consumer < T > for MapWithConsumer < 'f , C , U , F > where C : Consumer < R > , U : Send + Clone , F : Fn (& mut U , T) -> R + Sync , R : Send , { type Folder = MapWithFolder < 'f , C :: Folder , U , F > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (MapWithConsumer :: new (left , self . item . clone () , self . map_op) , MapWithConsumer :: new (right , self . item , self . map_op) , reducer ,) } fn into_folder (self) -> Self :: Folder { MapWithFolder { base : self . base . into_folder () , item : self . item , map_op : self . map_op , } } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_701!();