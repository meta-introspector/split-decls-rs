macro_rules! CastTo {
    () => {
        pub trait CastTo { type Target ; }
    };
}

CastTo!()