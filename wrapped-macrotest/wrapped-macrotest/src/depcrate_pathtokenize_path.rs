// Generated macro for tokenize_path (macro)
macro_rules! Depcrate_pathtokenize_path {
() => {
// Module: crate::path
// Provides: {"tokenize_path"}
// Dependencies: {}
macro_rules ! tokenize_path { ([$ (($ ($ component : tt) +)) *] [$ ($ cur : tt) +] / $ ($ rest : tt) +) => { tokenize_path ! ([$ (($ ($ component) +)) * ($ ($ cur) +)] [] $ ($ rest) +) } ; ([$ (($ ($ component : tt) +)) *] [$ ($ cur : tt) *] $ first : tt $ ($ rest : tt) *) => { tokenize_path ! ([$ (($ ($ component) +)) *] [$ ($ cur) * $ first] $ ($ rest) *) } ; ([$ (($ ($ component : tt) +)) *] [$ ($ cur : tt) +]) => { tokenize_path ! ([$ (($ ($ component) +)) * ($ ($ cur) +)]) } ; ([$ (($ ($ component : tt) +)) *]) => { { let mut path = std :: path :: PathBuf :: new () ; $ (path . push (& ($ ($ component) +)) ;) * path } } ; }
};
}
