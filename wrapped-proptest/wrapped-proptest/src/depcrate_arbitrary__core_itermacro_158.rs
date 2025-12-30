// Generated macro for macro_158 (macro)
macro_rules! Depcrate_arbitrary__core_itermacro_158 {
() => {
// Module: crate::arbitrary::_core::iter
// Provides: {"macro_158"}
// Dependencies: {}
lift1 ! ([fmt :: Debug + 'static + Iterator , B : 'static + Arbitrary + Iterator] Zip < B , A >, B :: Parameters ; base , args => (any_with ::< B > (args) , base) . prop_map (| (b , a) | b . zip (a)) . boxed ()) ;
};
}
