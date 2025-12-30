// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
impl bindings :: IComposableFactory_Impl for ComposableFactory_Impl { fn CreateInstance (& self , base : Ref < windows_core :: IInspectable > , inner : OutRef < windows_core :: IInspectable > ,) -> Result < bindings :: Composable > { _ = inner . write (None) ; if base . is_some () { Err (CLASS_E_NOAGGREGATION . into ()) } else { Ok (Composable :: new (0) . into ()) } } fn WithValue (& self , arg : i32 , base : Ref < windows_core :: IInspectable > , inner : OutRef < windows_core :: IInspectable > ,) -> Result < bindings :: Composable > { _ = inner . write (None) ; if base . is_some () { Err (CLASS_E_NOAGGREGATION . into ()) } else { Ok (Composable :: new (arg) . into ()) } } }
};
}
