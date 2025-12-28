macro_rules! deps {
    () => {
        ByteDestination!();
    };
}

macro_rules! ByteThreeHandle {
    () => {
        deps!();
        pub struct ByteThreeHandle < 'a , 'b > where 'b : 'a , { dest : & 'a mut ByteDestination < 'b > , }
    };
}

ByteThreeHandle!();