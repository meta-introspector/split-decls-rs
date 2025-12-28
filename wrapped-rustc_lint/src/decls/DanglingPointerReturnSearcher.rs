macro_rules! deps {
    () => {
        DanglingPointerLocalContext!();
        LateContext!();
    };
}

macro_rules! DanglingPointerReturnSearcher {
    () => {
        deps!();
        struct DanglingPointerReturnSearcher < 'lcx , 'tcx > { cx : & 'lcx LateContext < 'tcx > , dcx : & 'lcx DanglingPointerLocalContext < 'tcx > , }
    };
}

DanglingPointerReturnSearcher!()