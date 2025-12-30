// Generated macro for impl_9 (impl)
macro_rules! Depcrate_size_hintimpl_9 {
() => {
// Module: crate::size_hint
// Provides: {"impl_9"}
// Dependencies: {}
# [doc = " Perfectly adds two `SizeHint'`s"] impl core :: ops :: Add for SizeHint { type Output = SizeHint ; fn add (self , rhs : Self) -> Self :: Output { SizeHint { lower : self . lower () + rhs . lower () , upper : self . upper () . and_then (| this | rhs . upper () . map (| rhs | this + rhs)) , } } }
};
}
