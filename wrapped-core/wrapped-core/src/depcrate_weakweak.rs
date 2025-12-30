// Generated macro for Weak (struct)
macro_rules! Depcrate_weakWeak {
() => {
// Module: crate::weak
// Provides: {"Weak"}
// Dependencies: {}
# [doc = " `Weak` holds a non-owning reference to an object."] # [derive (Clone , PartialEq , Eq , Default)] pub struct Weak < I : Interface > (Option < imp :: IWeakReference > , PhantomData < I >) ;
};
}
