macro_rules! deps {
    () => {
        Reader!();
        Table!();
    };
}

macro_rules! File {
    () => {
        deps!();
        pub struct File { pub (crate) reader : * const Reader , bytes : Vec < u8 > , strings : usize , blobs : usize , tables : [Table ; 17] , }
    };
}

File!();