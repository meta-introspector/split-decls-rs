macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! read {
    () => {
        deps!();
        # [doc = ""] pub mod read { pub use gix_shallow :: read :: Error ; }
    };
}

read!()