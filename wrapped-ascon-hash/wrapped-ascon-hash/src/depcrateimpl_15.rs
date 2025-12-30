// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < P : HashParameters > Default for HashCore < P > { fn default () -> Self { Self { state : State :: new (P :: IV0 , P :: IV1 , P :: IV2 , P :: IV3 , P :: IV4) , phantom : PhantomData , } } }
};
}
