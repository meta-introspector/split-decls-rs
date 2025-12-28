macro_rules! deps {
    () => {
        Targets!();
        Sources!();
    };
}

macro_rules! IfThisChanged {
    () => {
        deps!();
        struct IfThisChanged < 'tcx > { tcx : TyCtxt < 'tcx > , if_this_changed : Sources , then_this_would_need : Targets , }
    };
}

IfThisChanged!();