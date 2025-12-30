// Generated macro for Domain (struct)
macro_rules! Depcrate_domainDomain {
() => {
// Module: crate::domain
// Provides: {"Domain"}
// Dependencies: {}
# [doc = " Representation of a single dimension of a function's domain."] # [derive (Clone , Debug)] pub struct Domain < T > { # [doc = " Start of the region for which a function is defined (ignoring poles)."] pub start : Bound < T > , # [doc = " Endof the region for which a function is defined (ignoring poles)."] pub end : Bound < T > , # [doc = " Additional points to check closer around. These can be e.g. undefined asymptotes or"] # [doc = " inflection points."] pub check_points : Option < fn () -> BoxIter < T > > , }
};
}
