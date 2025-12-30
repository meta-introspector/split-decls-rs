// Generated macro for tagname (function)
macro_rules! Depcrate_serializetagname {
() => {
// Module: crate::serialize
// Provides: {"tagname"}
// Dependencies: {}
fn tagname (name : & QualName) -> Atom { match name . ns { ns ! (html) | ns ! (mathml) | ns ! (svg) => () , ref ns => { warn ! ("node with weird namespace {:?}" , &* ns . 0) ; } } name . local . clone () }
};
}
