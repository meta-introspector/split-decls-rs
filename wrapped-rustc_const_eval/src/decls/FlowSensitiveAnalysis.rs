macro_rules! deps {
    () => {
        ConstCx!();
    };
}

macro_rules! FlowSensitiveAnalysis {
    () => {
        deps!();
        # [doc = " The dataflow analysis used to propagate qualifs on arbitrary CFGs."] pub (super) struct FlowSensitiveAnalysis < 'mir , 'tcx , Q > { ccx : & 'mir ConstCx < 'mir , 'tcx > , _qualif : PhantomData < Q > , }
    };
}

FlowSensitiveAnalysis!();