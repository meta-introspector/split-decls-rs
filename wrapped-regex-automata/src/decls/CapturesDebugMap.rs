macro_rules! deps {
    () => {
        Captures!();
        PatternID!();
    };
}

macro_rules! CapturesDebugMap {
    () => {
        deps!();
        # [doc = " A little helper type to provide a nice map-like debug representation for"] # [doc = " our capturing group spans."] struct CapturesDebugMap < 'a > { pid : PatternID , caps : & 'a Captures , }
    };
}

CapturesDebugMap!()