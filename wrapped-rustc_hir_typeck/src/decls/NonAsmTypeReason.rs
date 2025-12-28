macro_rules! NonAsmTypeReason {
    () => {
        enum NonAsmTypeReason < 'tcx > { UnevaluatedSIMDArrayLength (DefId , ty :: Const < 'tcx >) , Invalid (Ty < 'tcx >) , InvalidElement (DefId , Ty < 'tcx >) , NotSizedPtr (Ty < 'tcx >) , EmptySIMDArray (Ty < 'tcx >) , }
    };
}

NonAsmTypeReason!()