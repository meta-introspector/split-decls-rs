macro_rules! deps {
    () => {
        AutoStream!();
    };
}

macro_rules! Stdout {
    () => {
        deps!();
        # [doc = " An adaptive wrapper around the global standard output stream of the current process"] pub type Stdout = AutoStream < std :: io :: Stdout > ;
    };
}

Stdout!()