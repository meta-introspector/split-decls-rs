macro_rules! RcVec {
    () => {
        pub (crate) struct RcVec < T > { inner : Rc < Vec < T > > , }
    };
}

RcVec!()