macro_rules! deps {
    () => {
        ByteDestination!();
    };
}

macro_rules! ByteTwoHandle {
    () => {
        deps!();
        pub struct ByteTwoHandle < 'a , 'b > where 'b : 'a , { dest : & 'a mut ByteDestination < 'b > , }
    };
}

ByteTwoHandle!();