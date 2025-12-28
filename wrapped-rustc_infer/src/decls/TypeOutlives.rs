macro_rules! deps {
    () => {
        TypeOutlivesDelegate!();
        VerifyBoundCx!();
    };
}

macro_rules! TypeOutlives {
    () => {
        deps!();
        # [doc = " The `TypeOutlives` struct has the job of \"lowering\" a `T: 'a`"] # [doc = " obligation into a series of `'a: 'b` constraints and \"verify\"s, as"] # [doc = " described on the module comment. The final constraints are emitted"] # [doc = " via a \"delegate\" of type `D` -- this is usually the `infcx`, which"] # [doc = " accrues them into the `region_obligations` code, but for NLL we"] # [doc = " use something else."] pub struct TypeOutlives < 'cx , 'tcx , D > where D : TypeOutlivesDelegate < 'tcx > , { delegate : D , tcx : TyCtxt < 'tcx > , verify_bound : VerifyBoundCx < 'cx , 'tcx > , }
    };
}

TypeOutlives!()