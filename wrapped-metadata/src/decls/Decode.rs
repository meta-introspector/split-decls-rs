macro_rules! deps {
    () => {
        TypeIndex!();
    };
}

macro_rules! Decode {
    () => {
        deps!();
        pub trait Decode < 'a > { fn decode (index : & 'a TypeIndex , file : usize , code : usize) -> Self ; }
    };
}

Decode!();