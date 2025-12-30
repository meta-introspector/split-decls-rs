// Generated macro for impl_5 (impl)
macro_rules! Depcrate_bindingsimpl_5 {
() => {
// Module: crate::bindings
// Provides: {"impl_5"}
// Dependencies: {}
impl ITest { pub fn Input < P0 > (& self , input : P0) -> windows_core :: Result < i32 > where P0 : windows_core :: Param < ITest > , { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Input) (windows_core :: Interface :: as_raw (this) , input . param () . abi () , & mut result__ ,) . map (| | result__) } } pub fn Output (& self , value : i32 , output : & mut Option < ITest >) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Output) (windows_core :: Interface :: as_raw (this) , value , output as * mut _ as _ ,) . ok () } } pub fn Current (& self) -> windows_core :: Result < i32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Current) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn SetCurrent (& self , value : i32) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . SetCurrent) (windows_core :: Interface :: as_raw (this) , value ,) . ok () } } }
};
}
