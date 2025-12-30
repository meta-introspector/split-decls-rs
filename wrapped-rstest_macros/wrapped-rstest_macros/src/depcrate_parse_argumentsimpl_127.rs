// Generated macro for impl_127 (impl)
macro_rules! Depcrate_parse_argumentsimpl_127 {
() => {
// Module: crate::parse::arguments
// Provides: {"impl_127"}
// Dependencies: {}
impl ArgumentInfo { fn future (future : FutureArg) -> Self { Self { future , .. Default :: default () } } fn by_ref () -> Self { Self { by_ref : true , .. Default :: default () } } fn ignore () -> Self { Self { ignore : true , .. Default :: default () } } fn inner_pat (pat : Pat) -> Self { Self { inner_pat : Some (pat) , .. Default :: default () } } fn is_future (& self) -> bool { use FutureArg :: * ; matches ! (self . future , Define | Await) } fn is_future_await (& self) -> bool { use FutureArg :: * ; matches ! (self . future , Await) } fn is_by_ref (& self) -> bool { self . by_ref } fn is_ignore (& self) -> bool { self . ignore } }
};
}
