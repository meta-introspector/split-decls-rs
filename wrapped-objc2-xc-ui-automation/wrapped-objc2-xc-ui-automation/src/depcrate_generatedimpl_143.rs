// Generated macro for impl_143 (impl)
macro_rules! Depcrate_generatedimpl_143 {
() => {
// Module: crate::generated
// Provides: {"impl_143"}
// Dependencies: {}
impl XCUIDevice { extern_methods ! (# [doc = " The current device."] # [unsafe (method (sharedDevice))] # [unsafe (method_family = none)] pub fn sharedDevice (mtm : MainThreadMarker) -> Retained < XCUIDevice >; # [unsafe (method (new))] # [unsafe (method_family = new)] pub fn new (mtm : MainThreadMarker) -> Retained < Self >; # [deprecated] # [unsafe (method (init))] # [unsafe (method_family = init)] pub fn init (this : Allocated < Self >) -> Retained < Self >; # [doc = " The location currently being simulated by the device, if any."] # [unsafe (method (location))] # [unsafe (method_family = none)] pub fn location (& self) -> Option < Retained < XCUILocation >>; # [doc = " Setter for [`location`][Self::location]."] # [unsafe (method (setLocation :))] # [unsafe (method_family = none)] pub fn setLocation (& self , location : Option <& XCUILocation >) ; # [doc = " Get or set the UI style of the device. Uses the `XCUIDeviceAppearance` enum to describe the UI style."] # [unsafe (method (appearance))] # [unsafe (method_family = none)] pub fn appearance (& self) -> XCUIDeviceAppearance ; # [doc = " Setter for [`appearance`][Self::appearance]."] # [unsafe (method (setAppearance :))] # [unsafe (method_family = none)] pub fn setAppearance (& self , appearance : XCUIDeviceAppearance) ; # [doc = " Access system features of the device, such as its running applications, or the ability to open files on it."] # [unsafe (method (system))] # [unsafe (method_family = none)] pub fn system (& self) -> Retained < XCUISystem >;) ; }
};
}
