// Generated macro for macro_7192 (macro)
macro_rules! Depcrate_methodsmacro_7192 {
() => {
// Module: crate::methods
// Provides: {"macro_7192"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `&mut Mutex::lock` calls"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `Mutex::lock` is less efficient than"] # [doc = " calling `Mutex::get_mut`. In addition you also have a statically"] # [doc = " guarantee that the mutex isn't locked, instead of just a runtime"] # [doc = " guarantee."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::sync::{Arc, Mutex};"] # [doc = ""] # [doc = " let mut value_rc = Arc::new(Mutex::new(42_u8));"] # [doc = " let value_mutex = Arc::get_mut(&mut value_rc).unwrap();"] # [doc = ""] # [doc = " let mut value = value_mutex.lock().unwrap();"] # [doc = " *value += 1;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::sync::{Arc, Mutex};"] # [doc = ""] # [doc = " let mut value_rc = Arc::new(Mutex::new(42_u8));"] # [doc = " let value_mutex = Arc::get_mut(&mut value_rc).unwrap();"] # [doc = ""] # [doc = " let value = value_mutex.get_mut().unwrap();"] # [doc = " *value += 1;"] # [doc = " ```"] # [clippy :: version = "1.49.0"] pub MUT_MUTEX_LOCK , style , "`&mut Mutex::lock` does unnecessary locking" }
};
}
