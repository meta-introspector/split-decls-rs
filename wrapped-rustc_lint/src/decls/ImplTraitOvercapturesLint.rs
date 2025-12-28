macro_rules! ImplTraitOvercapturesLint {
    () => {
        struct ImplTraitOvercapturesLint < 'tcx > { uncaptured_spans : Vec < Span > , self_ty : Ty < 'tcx > , num_captured : usize , suggestion : Option < AddPreciseCapturingForOvercapture > , }
    };
}

ImplTraitOvercapturesLint!()