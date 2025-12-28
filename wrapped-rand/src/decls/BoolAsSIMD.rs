macro_rules! BoolAsSIMD {
    () => {
        pub (crate) trait BoolAsSIMD : Sized { fn any (self) -> bool ; }
    };
}

BoolAsSIMD!()