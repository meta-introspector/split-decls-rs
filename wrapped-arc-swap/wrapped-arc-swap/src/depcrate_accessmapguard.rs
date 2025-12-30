// Generated macro for MapGuard (struct)
macro_rules! Depcrate_accessMapGuard {
() => {
// Module: crate::access
// Provides: {"MapGuard"}
// Dependencies: {}
# [doc (hidden)] # [derive (Copy , Clone , Debug)] pub struct MapGuard < G , F , T , R > { guard : G , projection : F , _t : PhantomData < fn (& T) -> & R > , }
};
}
