// Generated macro for impl_44 (impl)
macro_rules! Depcrate_arcimpl_44 {
() => {
// Module: crate::arc
// Provides: {"impl_44"}
// Dependencies: {}
impl Arc < dyn Any + Send + Sync > { # [doc = " Attempts to downcast the `Arc<dyn Any + Send + Sync>` to a concrete type."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = " use std::any::Any;"] # [doc = ""] # [doc = " fn print_if_string(value: Arc<dyn Any + Send + Sync>) {"] # [doc = "     if let Ok(string) = value.downcast::<String>() {"] # [doc = "         println!(\"String ({}): {}\", string.len(), string);"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let my_string = \"Hello World\".to_string();"] # [doc = " print_if_string(Arc::from(Box::new(my_string) as Box<dyn Any + Send + Sync>));"] # [doc = " print_if_string(Arc::from(Box::new(0i8) as Box<dyn Any + Send + Sync>));"] # [doc = " // or with \"--cfg portable_atomic_unstable_coerce_unsized\" in RUSTFLAGS (requires Rust nightly):"] # [doc = " // print_if_string(Arc::new(my_string));"] # [doc = " // print_if_string(Arc::new(0i8));"] # [doc = " ```"] # [inline] pub fn downcast < T > (self) -> Result < Arc < T > , Self > where T : Any + Send + Sync , { if (* self) . is :: < T > () { unsafe { let ptr = Arc :: into_inner_non_null (self) ; Ok (Arc :: from_inner (ptr . cast :: < ArcInner < T > > ())) } } else { Err (self) } } }
};
}
