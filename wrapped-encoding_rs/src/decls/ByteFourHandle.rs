macro_rules! deps {
    () => {
        ByteDestination!();
    };
}

macro_rules! ByteFourHandle {
    () => {
        deps!();
        pub struct ByteFourHandle < 'a , 'b > where 'b : 'a , { dest : & 'a mut ByteDestination < 'b > , }
    };
}

ByteFourHandle!();