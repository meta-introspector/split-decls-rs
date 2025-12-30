// Generated macro for impl_686 (impl)
macro_rules! Depcrate_generatedimpl_686 {
() => {
// Module: crate::generated
// Provides: {"impl_686"}
// Dependencies: {}
impl NEVPNIKEv2PPKConfiguration { extern_methods ! (# [doc = " Initialize a newly-allocated NEVPNIKEv2PPKConfiguration object."] # [doc = ""] # [doc = " Parameter `identifier`: The identifier for the PPK."] # [doc = ""] # [doc = " Parameter `keychainReference`: A persistent reference to a keychain item of class kSecClassGenericPassword containing the PPK."] # [unsafe (method (initWithIdentifier : keychainReference :))] # [unsafe (method_family = init)] pub unsafe fn initWithIdentifier_keychainReference (this : Allocated < Self >, identifier : & NSString , keychain_reference : & NSData ,) -> Retained < Self >; # [doc = " The identifer for the PPK."] # [unsafe (method (identifier))] # [unsafe (method_family = none)] pub unsafe fn identifier (& self) -> Retained < NSString >; # [doc = " A persistent reference to a keychain item of class kSecClassGenericPassword containing the PPK."] # [unsafe (method (keychainReference))] # [unsafe (method_family = none)] pub unsafe fn keychainReference (& self) -> Retained < NSData >; # [doc = " Boolean indicating whether use of the PPK is mandatory or not. Default is YES."] # [unsafe (method (isMandatory))] # [unsafe (method_family = none)] pub unsafe fn isMandatory (& self) -> bool ; # [doc = " Setter for [`isMandatory`][Self::isMandatory]."] # [unsafe (method (setIsMandatory :))] # [unsafe (method_family = none)] pub unsafe fn setIsMandatory (& self , is_mandatory : bool) ;) ; }
};
}
