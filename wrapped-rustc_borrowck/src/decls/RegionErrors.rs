macro_rules! deps {
    () => {
        RegionErrorKind!();
    };
}

macro_rules! RegionErrors {
    () => {
        deps!();
        # [doc = " A collection of errors encountered during region inference. This is needed to efficiently"] # [doc = " report errors after borrow checking."] # [doc = ""] # [doc = " Usually we expect this to either be empty or contain a small number of items, so we can avoid"] # [doc = " allocation most of the time."] pub (crate) struct RegionErrors < 'tcx > (Vec < (RegionErrorKind < 'tcx > , ErrorGuaranteed) > , TyCtxt < 'tcx >) ;
    };
}

RegionErrors!()