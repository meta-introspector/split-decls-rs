macro_rules! deps {
    () => {
        Checker!();
    };
}

macro_rules! CheckLiveDrops {
    () => {
        deps!();
        struct CheckLiveDrops < 'mir , 'tcx > { checker : Checker < 'mir , 'tcx > , }
    };
}

CheckLiveDrops!();