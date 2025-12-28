macro_rules! deps {
    () => {
        TypeOutlives!();
        RegionBoundPairs!();
    };
}

macro_rules! VerifyBoundCx {
    () => {
        deps!();
        # [doc = " The `TypeOutlives` struct has the job of \"lowering\" a `T: 'a`"] # [doc = " obligation into a series of `'a: 'b` constraints and \"verifys\", as"] # [doc = " described on the module comment. The final constraints are emitted"] # [doc = " via a \"delegate\" of type `D` -- this is usually the `infcx`, which"] # [doc = " accrues them into the `region_obligations` code, but for NLL we"] # [doc = " use something else."] pub (crate) struct VerifyBoundCx < 'cx , 'tcx > { tcx : TyCtxt < 'tcx > , region_bound_pairs : & 'cx RegionBoundPairs < 'tcx > , # [doc = " During borrowck, if there are no outlives bounds on a generic"] # [doc = " parameter `T`, we assume that `T: 'in_fn_body` holds."] # [doc = ""] # [doc = " Outside of borrowck the only way to prove `T: '?0` is by"] # [doc = " setting  `'?0` to `'empty`."] implicit_region_bound : Option < ty :: Region < 'tcx > > , caller_bounds : & 'cx [ty :: PolyTypeOutlivesPredicate < 'tcx >] , }
    };
}

VerifyBoundCx!();