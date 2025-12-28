macro_rules! deps {
    () => {
        LateLintPassObject!();
    };
}

macro_rules! RuntimeCombinedLateLintPass {
    () => {
        deps!();
        struct RuntimeCombinedLateLintPass < 'a , 'tcx > { passes : & 'a mut [LateLintPassObject < 'tcx >] , }
    };
}

RuntimeCombinedLateLintPass!()