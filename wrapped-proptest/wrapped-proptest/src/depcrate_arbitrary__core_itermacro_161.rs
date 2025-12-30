// Generated macro for macro_161 (macro)
macro_rules! Depcrate_arbitrary__core_itermacro_161 {
() => {
// Module: crate::arbitrary::_core::iter
// Provides: {"macro_161"}
// Dependencies: {}
lift1 ! ([fmt :: Debug + 'static + Iterator < Item = T >, B : 'static + Arbitrary + Iterator < Item = T >, T] Chain < B , A >, B :: Parameters ; base , args => (any_with ::< B > (args) , base) . prop_map (| (b , a) | b . chain (a)) . boxed ()) ;
};
}
