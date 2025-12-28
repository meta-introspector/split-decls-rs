macro_rules! deps {
    () => {
        AutoderefSnapshot!();
    };
}

macro_rules! Autoderef {
    () => {
        deps!();
        # [doc = " Recursively dereference a type, considering both built-in"] # [doc = " dereferences (`*`) and the `Deref` trait."] # [doc = " Although called `Autoderef` it can be configured to use the"] # [doc = " `Receiver` trait instead of the `Deref` trait."] pub struct Autoderef < 'a , 'tcx > { infcx : & 'a InferCtxt < 'tcx > , span : Span , body_id : LocalDefId , param_env : ty :: ParamEnv < 'tcx > , state : AutoderefSnapshot < 'tcx > , include_raw_pointers : bool , use_receiver_trait : bool , silence_errors : bool , }
    };
}

Autoderef!();