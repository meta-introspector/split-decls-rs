macro_rules! LateLintPassObject {
    () => {
        pub (crate) type LateLintPassObject < 'tcx > = Box < dyn LateLintPass < 'tcx > + 'tcx > ;
    };
}

LateLintPassObject!()