macro_rules! deps {
    () => {
        ByteSource!();
    };
}

macro_rules! ByteUnreadHandle {
    () => {
        deps!();
        pub struct ByteUnreadHandle < 'a , 'b > where 'b : 'a , { source : & 'a mut ByteSource < 'b > , }
    };
}

ByteUnreadHandle!()