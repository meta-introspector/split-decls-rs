// Generated macro for SplitIntoParts (trait)
macro_rules! DepcrateSplitIntoParts {
() => {
// Module: crate
// Provides: {"SplitIntoParts"}
// Dependencies: {}
# [doc = " *(internal)* Split a part into nested parts."] # [doc = ""] # [doc = " This is used to implement splitting of nested parts."] pub unsafe trait SplitIntoParts < 'a , ContainingPart , Reference : PartialRef < 'a > > { # [doc = " A partial reference that has all the parts `Reference` and all parts of `Self` nested in"] # [doc = " `ContainingPart` as constant parts."] type Result : PartialRef < 'a , Target = Reference :: Target > ; # [doc = " A partial reference that has all the parts `Reference` and all parts of `Self` nested in"] # [doc = " `ContainingPart` as mutable parts."] type ResultMut : PartialRef < 'a , Target = Reference :: Target > ; }
};
}
