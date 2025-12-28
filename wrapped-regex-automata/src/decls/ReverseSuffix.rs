macro_rules! deps {
    () => {
        Core!();
        Prefilter!();
    };
}

macro_rules! ReverseSuffix {
    () => {
        deps!();
        # [derive (Debug)] struct ReverseSuffix { core : Core , pre : Prefilter , }
    };
}

ReverseSuffix!();