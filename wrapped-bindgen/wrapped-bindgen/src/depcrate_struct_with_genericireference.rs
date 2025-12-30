// Generated macro for IReference (struct)
macro_rules! Depcrate_struct_with_genericIReference {
() => {
// Module: crate::struct_with_generic
// Provides: {"IReference"}
// Dependencies: {}
# [repr (transparent)] # [derive (Clone , Debug , Eq , PartialEq)] pub struct IReference < T > (windows_core :: IUnknown , core :: marker :: PhantomData < T >) where T : windows_core :: RuntimeType + 'static ;
};
}
