macro_rules! Union {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Union { pub (crate) id : UnionId , }
    };
}

Union!();