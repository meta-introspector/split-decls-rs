// Generated macro for RelaxedBoundPolicy (enum)
macro_rules! DepcrateRelaxedBoundPolicy {
() => {
// Module: crate
// Provides: {"RelaxedBoundPolicy"}
// Dependencies: {}
# [doc = " How relaxed bounds `?Trait` should be treated."] # [doc = ""] # [doc = " Relaxed bounds should only be allowed in places where we later"] # [doc = " (namely during HIR ty lowering) perform *sized elaboration*."] # [derive (Clone , Copy , Debug)] enum RelaxedBoundPolicy < 'a > { Allowed , AllowedIfOnTyParam (NodeId , & 'a [ast :: GenericParam]) , Forbidden (RelaxedBoundForbiddenReason) , }
};
}
