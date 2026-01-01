// SRC: ../rust/compiler/rustc_thread_pool/src/tlv.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */
// Allows access to the Rayon's thread local value
// which is preserved when moving jobs across threads

use std::cell::Cell;
use std::ptr;

thread_local!(pub static TLV: Cell<*const ()> = const { Cell::new(ptr::null()) });
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=10 */

#[derive(Copy, Clone)]
pub(crate) struct Tlv(pub(crate) *const ());

impl Tlv {
    #[inline]
    pub(crate) fn null() -> Self {
        Self(ptr::null())
    }
}
/* AST_META: AST_ID=3 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=2 */

unsafe impl Sync for Tlv {}
/* AST_META: AST_ID=4 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl Send for Tlv {}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

/// Sets the current thread-local value
#[inline]
pub(crate) fn set(value: Tlv) {
    TLV.with(|tlv| tlv.set(value.0));
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

/// Returns the current thread-local value
#[inline]
pub(crate) fn get() -> Tlv {
    TLV.with(|tlv| Tlv(tlv.get()))
}