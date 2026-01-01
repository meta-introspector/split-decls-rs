// SRC: ../rust/compiler/rustc_thread_pool/src/compile_fail/rc_return.rs
/** ```compile_fail,E0277

use std::rc::Rc;

crate::rustc_thread_pool::join(|| Rc::new(22), || ()); //~ ERROR

``` */
mod left {}

/** ```compile_fail,E0277

use std::rc::Rc;

crate::rustc_thread_pool::join(|| (), || Rc::new(23)); //~ ERROR

``` */
mod right {}