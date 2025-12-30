// Generated macro for RelativeToErrorKind (enum)
macro_rules! Depcrate_path_extRelativeToErrorKind {
() => {
// Module: crate::path_ext
// Provides: {"RelativeToErrorKind"}
// Dependencies: {}
# [doc = " Error kind for [`RelativeToError`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] # [non_exhaustive] enum RelativeToErrorKind { # [doc = " Non-utf8 component in path."] NonUtf8 , # [doc = " Mismatching path prefixes."] PrefixMismatch , # [doc = " A provided path is ambiguous, in that there is no way to determine which"] # [doc = " components should be added from one path to the other to traverse it."] # [doc = ""] # [doc = " For example, `.` is ambiguous relative to `../..` because we don't know"] # [doc = " the names of the components being traversed."] AmbiguousTraversal , # [doc = " This is a catch-all error since we don't control the `std::path` API a"] # [doc = " Components iterator might decide (intentionally or not) to produce"] # [doc = " components which violates its own contract."] # [doc = ""] # [doc = " In particular we rely on only relative components being produced after"] # [doc = " the absolute prefix has been consumed."] IllegalComponent , }
};
}
