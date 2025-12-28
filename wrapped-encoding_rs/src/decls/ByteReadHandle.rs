macro_rules! deps {
    () => {
        ByteSource!();
    };
}

macro_rules! ByteReadHandle {
    () => {
        deps!();
        pub struct ByteReadHandle < 'a , 'b > where 'b : 'a , { source : & 'a mut ByteSource < 'b > , }
    };
}

ByteReadHandle!();