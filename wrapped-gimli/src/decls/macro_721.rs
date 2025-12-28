macro_rules! deps {
    () => {
        FrameTable!();
    };
}

macro_rules! macro_721 {
    () => {
        deps!();
        define_id ! (CieId , "An identifier for a CIE in a `FrameTable`.") ;
    };
}

macro_721!();