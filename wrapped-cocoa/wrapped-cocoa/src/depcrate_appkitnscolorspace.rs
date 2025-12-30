// Generated macro for NSColorSpace (trait)
macro_rules! Depcrate_appkitNSColorSpace {
() => {
// Module: crate::appkit
// Provides: {"NSColorSpace"}
// Dependencies: {}
pub trait NSColorSpace : Sized { unsafe fn deviceRGBColorSpace (_ : Self) -> id ; unsafe fn genericRGBColorSpace (_ : Self) -> id ; unsafe fn deviceCMYKColorSpace (_ : Self) -> id ; unsafe fn genericCMYKColorSpace (_ : Self) -> id ; unsafe fn deviceGrayColorSpace (_ : Self) -> id ; unsafe fn genericGrayColorSpace (_ : Self) -> id ; unsafe fn sRGBColorSpace (_ : Self) -> id ; unsafe fn extendedSRGBColorSpace (_ : Self) -> id ; unsafe fn displayP3ColorSpace (_ : Self) -> id ; unsafe fn genericGamma22GrayColorSpace (_ : Self) -> id ; unsafe fn extendedGenericGamma22GrayColorSpace (_ : Self) -> id ; unsafe fn adobeRGB1998ColorSpace (_ : Self) -> id ; unsafe fn alloc (_ : Self) -> id ; unsafe fn initWithCGColorSpace_ (self , cg_color_space : * const c_void ,) -> id ; unsafe fn CGColorSpace (self) -> * const c_void ; unsafe fn localizedName (self) -> id ; }
};
}
