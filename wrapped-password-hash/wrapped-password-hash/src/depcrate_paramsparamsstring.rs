// Generated macro for ParamsString (struct)
macro_rules! Depcrate_paramsParamsString {
() => {
// Module: crate::params
// Provides: {"ParamsString"}
// Dependencies: {}
# [doc = " Algorithm parameter string."] # [doc = ""] # [doc = " The [PHC string format specification][1] defines a set of optional"] # [doc = " algorithm-specific name/value pairs which can be encoded into a"] # [doc = " PHC-formatted parameter string as follows:"] # [doc = ""] # [doc = " ```text"] # [doc = " $<param>=<value>(,<param>=<value>)*"] # [doc = " ```"] # [doc = ""] # [doc = " This type represents that set of parameters."] # [doc = ""] # [doc = " [1]: https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#specification"] # [derive (Clone , Default , Eq , PartialEq)] pub struct ParamsString (Buffer) ;
};
}
