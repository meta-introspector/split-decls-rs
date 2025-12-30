// Generated macro for ForkByErrorPredicate (trait)
macro_rules! Depcrate_fork_predicatesForkByErrorPredicate {
() => {
// Module: crate::fork::predicates
// Provides: {"ForkByErrorPredicate"}
// Dependencies: {}
# [doc = " The predicate trait used by [`ForkByErrorProvider`]."] # [doc = ""] # [doc = " [`ForkByErrorProvider`]: super::ForkByErrorProvider"] pub trait ForkByErrorPredicate { # [doc = " The error to return if there are zero providers."] const UNIT_ERROR : DataErrorKind = DataErrorKind :: MarkerNotFound ; # [doc = " This function is called when a data request fails and there are additional providers"] # [doc = " that could possibly fulfill the request."] # [doc = ""] # [doc = " Arguments:"] # [doc = ""] # [doc = " - `&self` = Reference to the struct implementing the trait (for data capture)"] # [doc = " - `marker` = The [`DataMarkerInfo`] associated with the request"] # [doc = " - `req` = The [`DataRequest`]. This may be `None` if there is no request, such as"] # [doc = "   inside [`IterableDynamicDataProvider`]."] # [doc = " - `err` = The error that occurred."] # [doc = ""] # [doc = " Return value:"] # [doc = ""] # [doc = " - `true` to discard the error and attempt the request with the next provider."] # [doc = " - `false` to return the error and not perform any additional requests."] fn test (& self , marker : DataMarkerInfo , req : Option < DataRequest > , err : DataError) -> bool ; }
};
}
