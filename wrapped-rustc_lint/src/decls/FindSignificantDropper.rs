macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! FindSignificantDropper {
    () => {
        deps!();
        struct FindSignificantDropper < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , }
    };
}

FindSignificantDropper!();