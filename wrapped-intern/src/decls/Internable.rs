macro_rules! deps {
    () => {
        InternStorage!();
    };
}

macro_rules! Internable {
    () => {
        deps!();
        pub trait Internable : Hash + Eq + 'static { fn storage () -> & 'static InternStorage < Self > ; }
    };
}

Internable!()