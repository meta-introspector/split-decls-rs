// Generated macro for const_repeat (macro)
macro_rules! Depcrateconst_repeat {
() => {
// Module: crate
// Provides: {"const_repeat"}
// Dependencies: {}
macro_rules ! const_repeat { ([16] $ e : expr , $ T : ty) => { { $ e ; $ e ; $ e ; $ e ; $ e ; $ e ; $ e ; $ e ; $ e ; $ e ; $ e ; $ e ; $ e ; $ e ; $ e ; $ e } } ; ([1] $ e : expr , $ T : ty) => { { $ e } } ; ([16 $ ($ n : tt) *] $ e : expr , $ T : ty) => { { const fn e (_ : u32) -> $ T { const_repeat ! ([$ ($ n) *] $ e , $ T) } e (0) ; e (0) ; e (0) ; e (0) ; e (0) ; e (0) ; e (0) ; e (0) ; e (0) ; e (0) ; e (0) ; e (0) ; e (0) ; e (0) ; e (0) ; e (0) } } ; ([8 $ ($ n : tt) *] $ e : expr , $ T : ty) => { { const fn e (_ : u32) -> $ T { const_repeat ! ([$ ($ n) *] $ e , $ T) } e (0) ; e (0) ; e (0) ; e (0) ; e (0) ; e (0) ; e (0) ; e (0) } } ; ([4 $ ($ n : tt) *] $ e : expr , $ T : ty) => { { const fn e (_ : u32) -> $ T { const_repeat ! ([$ ($ n) *] $ e , $ T) } e (0) ; e (0) ; e (0) ; e (0) } } ; ([2 $ ($ n : tt) *] $ e : expr , $ T : ty) => { { const fn e (_ : u32) -> $ T { const_repeat ! ([$ ($ n) *] $ e , $ T) } e (0) ; e (0) } } ; }
};
}
