// Generated macro for leap (macro)
macro_rules! Depcrateleap {
() => {
// Module: crate
// Provides: {"leap"}
// Dependencies: {}
# [doc = " We can't use `?` in const contexts yet, so this macro acts"] # [doc = " as a workaround."] macro_rules ! leap { ($ x : expr) => { { match ($ x) { Some (val) => val , None => return None , } } } ; }
};
}
