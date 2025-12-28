macro_rules! WfCheckingCtxt {
    () => {
        pub (super) struct WfCheckingCtxt < 'a , 'tcx > { pub (super) ocx : ObligationCtxt < 'a , 'tcx , FulfillmentError < 'tcx > > , body_def_id : LocalDefId , param_env : ty :: ParamEnv < 'tcx > , }
    };
}

WfCheckingCtxt!();