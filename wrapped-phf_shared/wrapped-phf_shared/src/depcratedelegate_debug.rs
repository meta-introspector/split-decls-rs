// Generated macro for delegate_debug (macro)
macro_rules! Depcratedelegate_debug {
() => {
// Module: crate
// Provides: {"delegate_debug"}
// Dependencies: {}
# [doc = " Create an impl of `FmtConst` delegating to `fmt::Debug` for types that can deal with it."] # [doc = ""] # [doc = " Ideally with specialization this could be just one default impl and then specialized where"] # [doc = " it doesn't apply."] macro_rules ! delegate_debug (($ ty : ty) => { impl FmtConst for $ ty { fn fmt_const (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { write ! (f , "{:?}" , self) } } }) ;
};
}
