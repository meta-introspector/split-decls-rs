// Generated macro for tagname (function)
macro_rules! Depcrate_serializetagname {
() => {
// Module: crate::serialize
// Provides: {"tagname"}
// Dependencies: {}
fn tagname (name : & QualName) -> LocalName { match name . ns { ns ! (html) | ns ! (mathml) | ns ! (svg) => () , ref ns => { warn ! ("node with weird namespace {:?}" , ns) ; } , } name . local . clone () }
};
}
