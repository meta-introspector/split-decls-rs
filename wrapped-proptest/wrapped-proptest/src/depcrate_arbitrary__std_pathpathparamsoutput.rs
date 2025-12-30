// Generated macro for PathParamsOutput (struct)
macro_rules! Depcrate_arbitrary__std_pathPathParamsOutput {
() => {
// Module: crate::arbitrary::_std::path
// Provides: {"PathParamsOutput"}
// Dependencies: {}
# [doc = " A private type (not actually pub) representing the output of [`PathParams`] that can't be"] # [doc = " referred to by API users."] # [doc = ""] # [doc = " The goal of this type is to encapsulate the output of `PathParams`. If this layer weren't"] # [doc = " present, the type of `<PathBuf as Arbitrary>::Strategy` would be `SMapped<(bool, Vec<String>),"] # [doc = " Self>`. This is a problem because it exposes the internal representation of `PathParams` as an"] # [doc = " API. For example, if an additional parameter of randomness (e.g. another bool) were added, the"] # [doc = " type of `Strategy` would change."] # [doc = ""] # [doc = " With `PathParamsOutput`, the type of `Strategy` is `SMapped<PathParamsOutput, Self>`, which is a"] # [doc = " type that can't be named directly---only via `<PathBuf as Arbitrary>::Strategy`. The internal"] # [doc = " representation of `PathParams` can be changed without affecting the API."] # [derive (Debug)] pub struct PathParamsOutput { is_absolute : bool , components : Vec < String > , }
};
}
