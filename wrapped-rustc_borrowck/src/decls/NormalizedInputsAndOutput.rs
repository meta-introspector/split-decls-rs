macro_rules! NormalizedInputsAndOutput {
    () => {
        # [doc = " As part of computing the free region relations, we also have to"] # [doc = " normalize the input-output types, which we then need later. So we"] # [doc = " return those. This vector consists of first the input types and"] # [doc = " then the output type as the last element."] type NormalizedInputsAndOutput < 'tcx > = Vec < Ty < 'tcx > > ;
    };
}

NormalizedInputsAndOutput!();