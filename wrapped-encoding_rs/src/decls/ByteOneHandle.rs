macro_rules! deps {
    () => {
        ByteDestination!();
    };
}

macro_rules! ByteOneHandle {
    () => {
        deps!();
        pub struct ByteOneHandle < 'a , 'b > where 'b : 'a , { dest : & 'a mut ByteDestination < 'b > , }
    };
}

ByteOneHandle!();