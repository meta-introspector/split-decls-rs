macro_rules! deps {
    () => {
        AutoStream!();
    };
}

macro_rules! Stderr {
    () => {
        deps!();
        # [doc = " An adaptive wrapper around the global standard error stream of the current process"] pub type Stderr = AutoStream < std :: io :: Stderr > ;
    };
}

Stderr!()