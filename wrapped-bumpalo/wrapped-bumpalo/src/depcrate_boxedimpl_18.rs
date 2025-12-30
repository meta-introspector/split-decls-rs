// Generated macro for impl_18 (impl)
macro_rules! Depcrate_boxedimpl_18 {
() => {
// Module: crate::boxed
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'a > Box < 'a , dyn Any + Send > { # [inline] # [doc = " Attempt to downcast the box to a concrete type."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::any::Any;"] # [doc = ""] # [doc = " fn print_if_string(value: Box<dyn Any + Send>) {"] # [doc = "     if let Ok(string) = value.downcast::<String>() {"] # [doc = "         println!(\"String ({}): {}\", string.len(), string);"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let my_string = \"Hello World\".to_string();"] # [doc = " print_if_string(Box::new(my_string));"] # [doc = " print_if_string(Box::new(0i8));"] # [doc = " ```"] pub fn downcast < T : Any > (self) -> Result < Box < 'a , T > , Box < 'a , dyn Any + Send > > { if self . is :: < T > () { unsafe { let raw : * mut (dyn Any + Send) = Box :: into_raw (self) ; Ok (Box :: from_raw (raw as * mut T)) } } else { Err (self) } } }
};
}
