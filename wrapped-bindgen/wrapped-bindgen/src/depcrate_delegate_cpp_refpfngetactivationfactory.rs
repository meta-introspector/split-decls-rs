// Generated macro for PFNGETACTIVATIONFACTORY (type)
macro_rules! Depcrate_delegate_cpp_refPFNGETACTIVATIONFACTORY {
() => {
// Module: crate::delegate_cpp_ref
// Provides: {"PFNGETACTIVATIONFACTORY"}
// Dependencies: {}
pub type PFNGETACTIVATIONFACTORY = Option < unsafe extern "system" fn (param0 : windows_core :: Ref < windows_core :: HSTRING > , param1 : windows_core :: OutRef < IActivationFactory > ,) -> windows_core :: HRESULT , > ;
};
}
