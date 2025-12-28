macro_rules! deps {
    () => {
        SmallIndex!();
    };
}

macro_rules! CaptureNameMap {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] type CaptureNameMap = alloc :: collections :: BTreeMap < Arc < str > , SmallIndex > ;
    };
}

CaptureNameMap!()