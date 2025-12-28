macro_rules! InferredCaptureInformation {
    () => {
        # [doc = " Intermediate format to store a captured `Place` and associated `ty::CaptureInfo`"] # [doc = " during capture analysis. Information in this map feeds into the minimum capture"] # [doc = " analysis pass."] type InferredCaptureInformation < 'tcx > = Vec < (Place < 'tcx > , ty :: CaptureInfo) > ;
    };
}

InferredCaptureInformation!()