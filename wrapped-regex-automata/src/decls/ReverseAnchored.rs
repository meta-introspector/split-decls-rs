macro_rules! deps {
    () => {
        Core!();
    };
}

macro_rules! ReverseAnchored {
    () => {
        deps!();
        # [derive (Debug)] struct ReverseAnchored { core : Core , }
    };
}

ReverseAnchored!()