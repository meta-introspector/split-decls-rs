// Generated macro for impl_175 (impl)
macro_rules! Depcrate_generatedimpl_175 {
() => {
// Module: crate::generated
// Provides: {"impl_175"}
// Dependencies: {}
impl SRKeyboardMetrics { extern_methods ! (# [doc = " The duration over which these metrics were calculated"] # [unsafe (method (duration))] # [unsafe (method_family = none)] pub unsafe fn duration (& self) -> NSTimeInterval ; # [doc = " The identifier of the keyboard in the keyboard list"] # [unsafe (method (keyboardIdentifier))] # [unsafe (method_family = none)] pub unsafe fn keyboardIdentifier (& self) -> Retained < NSString >; # [doc = " The version of keyboard metrics"] # [unsafe (method (version))] # [unsafe (method_family = none)] pub unsafe fn version (& self) -> Retained < NSString >; # [doc = " The width of the keyboard in mm in the session"] # [unsafe (method (width))] # [unsafe (method_family = none)] pub unsafe fn width (& self) -> Retained < NSMeasurement < NSUnitLength >>; # [doc = " The height of the keyboard in mm in the session"] # [unsafe (method (height))] # [unsafe (method_family = none)] pub unsafe fn height (& self) -> Retained < NSMeasurement < NSUnitLength >>; # [doc = " The input modes used during a keyboard session"] # [unsafe (method (inputModes))] # [unsafe (method_family = none)] pub unsafe fn inputModes (& self) -> Retained < NSArray < NSString >>; # [doc = " The keyboard session identifiers. These are the identifiers of the keyboard sessions that contributed to keyboard metrics sample to correlate current stream with another stream using the same keyboard session indentifiers"] # [unsafe (method (sessionIdentifiers))] # [unsafe (method_family = none)] pub unsafe fn sessionIdentifiers (& self) -> Retained < NSArray < NSString >>;) ; }
};
}
