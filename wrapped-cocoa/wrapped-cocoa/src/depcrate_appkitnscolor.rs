// Generated macro for NSColor (trait)
macro_rules! Depcrate_appkitNSColor {
() => {
// Module: crate::appkit
// Provides: {"NSColor"}
// Dependencies: {}
pub trait NSColor : Sized { unsafe fn clearColor (_ : Self) -> id ; unsafe fn colorWithRed_green_blue_alpha_ (_ : Self , r : CGFloat , g : CGFloat , b : CGFloat , a : CGFloat ,) -> id ; unsafe fn colorWithSRGBRed_green_blue_alpha_ (_ : Self , r : CGFloat , g : CGFloat , b : CGFloat , a : CGFloat ,) -> id ; unsafe fn colorWithDeviceRed_green_blue_alpha_ (_ : Self , r : CGFloat , g : CGFloat , b : CGFloat , a : CGFloat ,) -> id ; unsafe fn colorWithDisplayP3Red_green_blue_alpha_ (_ : Self , r : CGFloat , g : CGFloat , b : CGFloat , a : CGFloat ,) -> id ; unsafe fn colorWithCalibratedRed_green_blue_alpha_ (_ : Self , r : CGFloat , g : CGFloat , b : CGFloat , a : CGFloat ,) -> id ; unsafe fn colorUsingColorSpace_ (self , color_space : id) -> id ; unsafe fn alphaComponent (self) -> CGFloat ; unsafe fn whiteComponent (self) -> CGFloat ; unsafe fn redComponent (self) -> CGFloat ; unsafe fn greenComponent (self) -> CGFloat ; unsafe fn blueComponent (self) -> CGFloat ; unsafe fn cyanComponent (self) -> CGFloat ; unsafe fn magentaComponent (self) -> CGFloat ; unsafe fn yellowComponent (self) -> CGFloat ; unsafe fn blackComponent (self) -> CGFloat ; unsafe fn hueComponent (self) -> CGFloat ; unsafe fn saturationComponent (self) -> CGFloat ; unsafe fn brightnessComponent (self) -> CGFloat ; }
};
}
