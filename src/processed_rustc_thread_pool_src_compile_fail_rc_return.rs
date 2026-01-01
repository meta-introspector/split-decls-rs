// SRC: ../rust/compiler/rustc_thread_pool/src/compile_fail/rc_return.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */
/** ```compile_fail,E0277

use std::rc::Rc;

crate::rustc_thread_pool::join(|| Rc::new(22), || ()); //~ ERROR

``` */
mod left {}
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */

/** ```compile_fail,E0277

use std::rc::Rc;

crate::rustc_thread_pool::join(|| (), || Rc::new(23)); //~ ERROR

``` */
mod right {}