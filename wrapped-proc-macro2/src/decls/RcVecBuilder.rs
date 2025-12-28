macro_rules! RcVecBuilder {
    () => {
        pub (crate) struct RcVecBuilder < T > { inner : Vec < T > , }
    };
}

RcVecBuilder!();