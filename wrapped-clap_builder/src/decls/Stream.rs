macro_rules! Stream {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub (crate) enum Stream { Stdout , Stderr , }
    };
}

Stream!()