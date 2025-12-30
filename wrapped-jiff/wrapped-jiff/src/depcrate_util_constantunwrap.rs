// Generated macro for unwrap (macro)
macro_rules! Depcrate_util_constantunwrap {
() => {
// Module: crate::util::constant
// Provides: {"unwrap"}
// Dependencies: {}
# [doc = " Unwrap an `Option<T>` in a `const` context."] # [doc = ""] # [doc = " If it fails, panics with the given message."] macro_rules ! unwrap { ($ val : expr , $ msg : expr $ (,) ?) => { match $ val { Some (val) => val , None => panic ! ($ msg) , } } ; }
};
}
