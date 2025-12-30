// Generated macro for Finish (trait)
macro_rules! Depcrate_internalFinish {
() => {
// Module: crate::internal
// Provides: {"Finish"}
// Dependencies: {}
# [doc = " Helper trait to convert a parser's result to a more manageable type"] pub trait Finish < I , O , E > { # [doc = " converts the parser's result to a type that is more consumable by error"] # [doc = " management libraries. It keeps the same `Ok` branch, and merges `Err::Error`"] # [doc = " and `Err::Failure` into the `Err` side."] # [doc = ""] # [doc = " *warning*: if the result is `Err(Err::Incomplete(_))`, this method will panic."] # [doc = " - \"complete\" parsers: It will not be an issue, `Incomplete` is never used"] # [doc = " - \"streaming\" parsers: `Incomplete` will be returned if there's not enough data"] # [doc = "   for the parser to decide, and you should gather more data before parsing again."] # [doc = "   Once the parser returns either `Ok(_)`, `Err(Err::Error(_))` or `Err(Err::Failure(_))`,"] # [doc = "   you can get out of the parsing loop and call `finish()` on the parser's result"] fn finish (self) -> Result < (I , O) , E > ; }
};
}
