macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! write {
    () => {
        deps!();
        # [doc = ""] pub mod write { pub use gix_shallow :: write :: Error ; }
    };
}

write!()