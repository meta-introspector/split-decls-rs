macro_rules! deps {
    () => {
        LateLintPassObject!();
    };
}

macro_rules! LateLintPassFactory {
    () => {
        deps!();
        type LateLintPassFactory = dyn for < 'tcx > Fn (TyCtxt < 'tcx >) -> LateLintPassObject < 'tcx > + sync :: DynSend + sync :: DynSync ;
    };
}

LateLintPassFactory!()