macro_rules! deps {
    () => {
        Writer!();
    };
}

macro_rules! new {
    () => {
        deps!();
        pub fn new < B > (buf : B) -> Writer < B > { Writer { buf } }
    };
}

new!();