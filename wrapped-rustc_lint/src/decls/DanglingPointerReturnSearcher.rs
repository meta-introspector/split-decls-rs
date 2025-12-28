macro_rules! deps {
    () => {
        LateContext!();
        DanglingPointerLocalContext!();
    };
}

macro_rules! DanglingPointerReturnSearcher {
    () => {
        deps!();
        struct DanglingPointerReturnSearcher < 'lcx , 'tcx > { cx : & 'lcx LateContext < 'tcx > , dcx : & 'lcx DanglingPointerLocalContext < 'tcx > , }
    };
}

DanglingPointerReturnSearcher!();