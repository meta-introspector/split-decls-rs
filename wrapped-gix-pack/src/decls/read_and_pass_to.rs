macro_rules! deps {
    () => {
        PassThrough!();
    };
}

macro_rules! read_and_pass_to {
    () => {
        deps!();
        fn read_and_pass_to < R : io :: Read , W : io :: Write > (read : & mut R , to : W) -> PassThrough < & mut R , W > { PassThrough { read , write : to } }
    };
}

read_and_pass_to!()