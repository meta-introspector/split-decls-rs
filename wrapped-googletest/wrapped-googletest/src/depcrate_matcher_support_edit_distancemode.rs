// Generated macro for Mode (enum)
macro_rules! Depcrate_matcher_support_edit_distanceMode {
() => {
// Module: crate::matcher_support::edit_distance
// Provides: {"Mode"}
// Dependencies: {}
# [doc = " Controls the termination condition of [`edit_list`]."] # [derive (Clone , Copy)] pub (crate) enum Mode { # [doc = " Indicates that the two arguments are intended to be equal."] # [doc = ""] # [doc = " The entire edit list to transform between `actual` and `expected` is"] # [doc = " returned."] Exact , # [doc = " Indicates that `expected` is inteded to be a prefix of `actual`."] # [doc = ""] # [doc = " Any additional parts of `actual` after the prefix `expected` are omitted"] # [doc = " from the output."] Prefix , # [doc = " Similar to [`Mode::Prefix`], except it is also assumed that `actual` has"] # [doc = " some number of initial lines which should not be in the output."] # [doc = ""] # [doc = " Any initial [`Edit::ExtraActual`] entries are replaced with"] # [doc = " [`Edit::AdditionalActual`] in the edit list. If the first entry which is"] # [doc = " not an [`Edit::ExtraActual`] is [`Edit::ExtraExpected`], then the last"] # [doc = " [`Edit::ExtraActual`] is actual in the output."] Contains , }
};
}
