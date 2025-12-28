macro_rules! deps {
    () => {
        LifetimeMismatchOpaqueParam!();
        RegionErrors!();
    };
}

macro_rules! DeferredOpaqueTypeError {
    () => {
        deps!();
        # [doc = " We defer errors from [fn handle_opaque_type_uses] and only report them"] # [doc = " if there are no `RegionErrors`. If there are region errors, it's likely"] # [doc = " that errors here are caused by them and don't need to be handled separately."] pub (crate) enum DeferredOpaqueTypeError < 'tcx > { InvalidOpaqueTypeArgs (NonDefiningUseReason < 'tcx >) , LifetimeMismatchOpaqueParam (LifetimeMismatchOpaqueParam < 'tcx >) , UnexpectedHiddenRegion { # [doc = " The opaque type."] opaque_type_key : OpaqueTypeKey < 'tcx > , # [doc = " The hidden type containing the member region."] hidden_type : OpaqueHiddenType < 'tcx > , # [doc = " The unexpected region."] member_region : Region < 'tcx > , } , NonDefiningUseInDefiningScope { span : Span , opaque_type_key : OpaqueTypeKey < 'tcx > , } , }
    };
}

DeferredOpaqueTypeError!();