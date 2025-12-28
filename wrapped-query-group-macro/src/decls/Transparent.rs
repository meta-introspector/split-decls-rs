macro_rules! Transparent {
    () => {
        pub (crate) struct Transparent { pub (crate) signature : syn :: Signature , pub (crate) pat_and_tys : Vec < PatType > , pub (crate) invoke : Option < Path > , pub (crate) default : Option < syn :: Block > , }
    };
}

Transparent!();