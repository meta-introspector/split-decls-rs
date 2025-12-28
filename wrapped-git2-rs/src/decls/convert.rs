macro_rules! deps {
    () => {
        Convert!();
    };
}

macro_rules! convert {
    () => {
        deps!();
        pub fn convert < T , U : Convert < T > > (u : & U) -> T { u . convert () }
    };
}

convert!()