macro_rules! PatternRefutability {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub (crate) enum PatternRefutability { Refutable , Irrefutable , }
    };
}

PatternRefutability!()