// Generated macro for InterfaceMethod (struct)
macro_rules! DepcrateInterfaceMethod {
() => {
// Module: crate
// Provides: {"InterfaceMethod"}
// Dependencies: {}
# [doc = " A parsed interface method"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " #[windows_interface::interface(\"8CEEB155-2849-4ce5-9448-91FF70E1E4D9\")]"] # [doc = " unsafe trait IUIAnimationVariable: IUnknown {"] # [doc = "     fn GetValue(&self, value: *mut f64) -> HRESULT;"] # [doc = "   //^ parses this"] # [doc = " }"] # [doc = " ```"] struct InterfaceMethod { pub name : syn :: Ident , pub visibility : syn :: Visibility , pub args : Vec < InterfaceMethodArg > , pub ret : syn :: ReturnType , pub docs : Vec < syn :: Attribute > , }
};
}
