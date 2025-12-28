macro_rules! deps {
    () => {
        RegionVariableValue!();
    };
}

macro_rules! RegionVidKey {
    () => {
        deps!();
        # [derive (PartialEq , Copy , Clone , Debug)] pub (crate) struct RegionVidKey < 'tcx > { pub vid : ty :: RegionVid , pub phantom : PhantomData < RegionVariableValue < 'tcx > > , }
    };
}

RegionVidKey!()