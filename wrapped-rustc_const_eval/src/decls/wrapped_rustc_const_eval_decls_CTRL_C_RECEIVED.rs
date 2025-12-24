use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// `rustc_driver::main` installs a handler that will set this to `true` if
/// the compiler has been sent a request to shut down, such as by a Ctrl-C.
/// This static lives here because it is only read by the interpreter.
pub static CTRL_C_RECEIVED: AtomicBool = AtomicBool::new(false);
