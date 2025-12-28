macro_rules! deps {
    () => {
        Obligation!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl < 'tcx , O > Obligation < 'tcx , O > { pub fn new (tcx : TyCtxt < 'tcx > , cause : ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , predicate : impl Upcast < TyCtxt < 'tcx > , O > ,) -> Obligation < 'tcx , O > { Self :: with_depth (tcx , cause , 0 , param_env , predicate) } # [doc = " We often create nested obligations without setting the correct depth."] # [doc = ""] # [doc = " To deal with this evaluate and fulfill explicitly update the depth"] # [doc = " of nested obligations using this function."] pub fn set_depth_from_parent (& mut self , parent_depth : usize) { self . recursion_depth = cmp :: max (parent_depth + 1 , self . recursion_depth) ; } pub fn with_depth (tcx : TyCtxt < 'tcx > , cause : ObligationCause < 'tcx > , recursion_depth : usize , param_env : ty :: ParamEnv < 'tcx > , predicate : impl Upcast < TyCtxt < 'tcx > , O > ,) -> Obligation < 'tcx , O > { let predicate = predicate . upcast (tcx) ; Obligation { cause , param_env , recursion_depth , predicate } } pub fn misc (tcx : TyCtxt < 'tcx > , span : Span , body_id : LocalDefId , param_env : ty :: ParamEnv < 'tcx > , trait_ref : impl Upcast < TyCtxt < 'tcx > , O > ,) -> Obligation < 'tcx , O > { Obligation :: new (tcx , ObligationCause :: misc (span , body_id) , param_env , trait_ref) } pub fn with < P > (& self , tcx : TyCtxt < 'tcx > , value : impl Upcast < TyCtxt < 'tcx > , P > ,) -> Obligation < 'tcx , P > { Obligation :: with_depth (tcx , self . cause . clone () , self . recursion_depth , self . param_env , value) } }
    };
}

impl_325!();