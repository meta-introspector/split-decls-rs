macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! c_try {
    () => {
        deps!();
        pub fn c_try (ret : libc :: c_int) -> Result < libc :: c_int , Error > { match ret { n if n < 0 => Err (last_error (n)) , n => Ok (n) , } }
    };
}

c_try!()