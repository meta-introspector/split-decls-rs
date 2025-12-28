macro_rules! deps {
    () => {
        TypeIndex!();
    };
}

macro_rules! Blob {
    () => {
        deps!();
        pub struct Blob < 'a > { index : & 'a TypeIndex , file : usize , slice : & 'a [u8] , }
    };
}

Blob!();