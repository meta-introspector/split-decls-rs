// Generated macro for impl_17 (impl)
macro_rules! Depcrate_boxedimpl_17 {
() => {
// Module: crate::boxed
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'a > Box < 'a , dyn Any > { # [inline] # [doc = " Attempt to downcast the box to a concrete type."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::any::Any;"] # [doc = ""] # [doc = " fn print_if_string(value: Box<dyn Any>) {"] # [doc = "     if let Ok(string) = value.downcast::<String>() {"] # [doc = "         println!(\"String ({}): {}\", string.len(), string);"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let my_string = \"Hello World\".to_string();"] # [doc = " print_if_string(Box::new(my_string));"] # [doc = " print_if_string(Box::new(0i8));"] # [doc = " ```"] pub fn downcast < T : Any > (self) -> Result < Box < 'a , T > , Box < 'a , dyn Any > > { if self . is :: < T > () { unsafe { let raw : * mut dyn Any = Box :: into_raw (self) ; Ok (Box :: from_raw (raw as * mut T)) } } else { Err (self) } } }
};
}
