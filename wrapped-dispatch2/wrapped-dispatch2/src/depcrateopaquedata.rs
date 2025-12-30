// Generated macro for OpaqueData (type)
macro_rules! DepcrateOpaqueData {
() => {
// Module: crate
// Provides: {"OpaqueData"}
// Dependencies: {}
type OpaqueData = UnsafeCell < PhantomData < (* const UnsafeCell < () > , PhantomPinned) > > ;
};
}
