macro_rules! ForceCaptureTraitArgs {
    () => {
        # [derive (Debug , Copy , Clone)] enum ForceCaptureTraitArgs { Yes , No , }
    };
}

ForceCaptureTraitArgs!();