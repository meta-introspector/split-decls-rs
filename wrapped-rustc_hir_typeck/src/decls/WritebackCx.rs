macro_rules! deps {
    () => {
        FnCtxt!();
    };
}

macro_rules! WritebackCx {
    () => {
        deps!();
        # [doc = " The Writeback context. This visitor walks the HIR, checking the"] # [doc = " fn-specific typeck results to find inference variables. It resolves"] # [doc = " those inference variables and writes the final result into the"] # [doc = " `TypeckResults`. It also applies a few ad-hoc checks that were not"] # [doc = " convenient to do elsewhere."] struct WritebackCx < 'cx , 'tcx > { fcx : & 'cx FnCtxt < 'cx , 'tcx > , typeck_results : ty :: TypeckResults < 'tcx > , body : & 'tcx hir :: Body < 'tcx > , rustc_dump_user_args : bool , }
    };
}

WritebackCx!();