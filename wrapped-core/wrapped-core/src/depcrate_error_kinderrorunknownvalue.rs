// Generated macro for ErrorUnknownValue (struct)
macro_rules! Depcrate_error_kindErrorUnknownValue {
() => {
// Module: crate::error::kind
// Provides: {"ErrorUnknownValue"}
// Dependencies: {}
# [doc = " An error where an unknown value was seen in a given position,"] # [doc = " with a possible \"did-you-mean\" suggestion to get the user back on the right track."] # [derive (Clone , Debug)] # [cfg_attr (test , derive (PartialEq))] pub (in crate :: error) struct ErrorUnknownValue { # [doc = " The thing whose value is unknown."] noun : UnknownValuePosition , value : String , # [doc = " The best suggestion of what field the caller could have meant, along with"] # [doc = " the similarity score between that best option and the actual caller-provided"] # [doc = " field name."] did_you_mean : Option < (f64 , String) > , # [doc = " Set of all known valid field names."] # [doc = ""] # [doc = " This is a `BTreeSet` so that names will be displayed in alphabetical order"] # [doc = " without needing display-time sorting."] alts : BTreeSet < String > , }
};
}
