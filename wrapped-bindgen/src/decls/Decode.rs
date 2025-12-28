macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! Decode {
    () => {
        deps!();
        pub trait Decode { fn decode (file : & 'static File , code : usize) -> Self ; }
    };
}

Decode!();