// Generated macro for Collect (struct)
macro_rules! Depcrate_collectCollect {
() => {
// Module: crate::collect
// Provides: {"Collect"}
// Dependencies: {}
# [doc = " A future which takes a list of futures and resolves with a vector of the"] # [doc = " completed values."] # [doc = ""] # [doc = " This future is created with the `collect` method."] pub struct Collect < I > where I : IntoIterator + Send + 'static , I :: Item : IntoFuture , I :: IntoIter : Send + 'static , { cur : Option < Collapsed < < I :: Item as IntoFuture > :: Future > > , remaining : I :: IntoIter , result : Vec < < I :: Item as IntoFuture > :: Item > , }
};
}
