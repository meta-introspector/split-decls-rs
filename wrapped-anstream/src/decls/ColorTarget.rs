macro_rules! ColorTarget {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum ColorTarget { Fg , Bg , Underline , }
    };
}

ColorTarget!()