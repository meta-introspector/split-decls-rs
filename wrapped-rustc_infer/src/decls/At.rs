macro_rules! deps {
    () => {
        InferCtxt!();
    };
}

macro_rules! At {
    () => {
        deps!();
        # [derive (Clone , Copy)] pub struct At < 'a , 'tcx > { pub infcx : & 'a InferCtxt < 'tcx > , pub cause : & 'a ObligationCause < 'tcx > , pub param_env : ty :: ParamEnv < 'tcx > , }
    };
}

At!();