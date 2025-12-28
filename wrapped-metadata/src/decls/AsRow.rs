macro_rules! deps {
    () => {
        File!();
        TypeIndex!();
        Decode!();
        Row!();
        Blob!();
        RowIterator!();
    };
}

macro_rules! AsRow {
    () => {
        deps!();
        pub trait AsRow < 'a > : Copy { const TABLE : usize ; fn to_row (& self) -> Row < 'a > ; fn from_row (row : Row < 'a >) -> Self ; fn index (& self) -> & 'a TypeIndex { let row = self . to_row () ; row . index } fn file (& self) -> & 'a File { let row = self . to_row () ; row . index . files (row . file) } fn pos (& self) -> usize { self . to_row () . pos } fn usize (& self , column : usize) -> usize { self . file () . usize (self . pos () , Self :: TABLE , column) } fn str (& self , column : usize) -> & 'a str { self . file () . str (self . pos () , Self :: TABLE , column) } fn row < R : AsRow < 'a > > (& self , column : usize) -> R { let row = self . to_row () ; R :: from_row (Row :: new (row . index , row . file , self . usize (column) - 1)) } fn decode < T : Decode < 'a > > (& self , column : usize) -> T { let row = self . to_row () ; T :: decode (row . index , row . file , self . usize (column)) } fn blob (& self , column : usize) -> Blob < 'a > { let row = self . to_row () ; Blob :: new (row . index , row . file , self . file () . blob (self . pos () , Self :: TABLE , column) ,) } fn list < R : AsRow < 'a > > (& self , column : usize) -> RowIterator < 'a , R > { let row = self . to_row () ; RowIterator :: new (row . index , row . file , self . file () . list (self . pos () , Self :: TABLE , column , R :: TABLE) ,) } fn equal_range < L : AsRow < 'a > > (& self , column : usize , value : usize) -> RowIterator < 'a , L > { let row = self . to_row () ; RowIterator :: new (row . index , row . file , self . file () . equal_range (L :: TABLE , column , value) ,) } fn parent_row < P : AsRow < 'a > > (& self , column : usize) -> P { let row = self . to_row () ; P :: from_row (Row :: new (row . index , row . file , self . file () . parent (self . pos () , P :: TABLE , column) ,)) } }
    };
}

AsRow!()