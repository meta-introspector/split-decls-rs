// SRC: ../rust/compiler/rustc_data_structures/src/lib.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=44 */
// Various data structures used by the Rust compiler. The intention
// is that code in here should not be *specific* to rustc, so that
// it can be easily unit tested and so forth.
//
// # Note
//
// This API is completely unstable and subject to change.

// tidy-alphabetical-start
#[allow(internal_features)]
#[allow(rustc::default_hash_types)]
#[allow(rustc::potential_query_instability)]
#[deny(unsafe_op_in_unsafe_fn)]
#[doc(html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")]
#[doc(rust_logo)]
#[feature(allocator_api)]
#[feature(array_windows)]
#[feature(ascii_char)]
#[feature(ascii_char_variants)]
#[feature(assert_matches)]
#[feature(auto_traits)]
#[feature(cfg_select)]
#[feature(core_intrinsics)]
#[feature(dropck_eyepatch)]
#[feature(extend_one)]
#[feature(file_buffered)]
#[feature(map_try_insert)]
#[feature(min_specialization)]
#[feature(negative_impls)]
#[feature(never_type)]
#[feature(ptr_alignment_type)]
#[feature(rustc_attrs)]
#[feature(rustdoc_internals)]
#[feature(sized_hierarchy)]
#[feature(test)]
#[feature(thread_id_value)]
#[feature(type_alias_impl_trait)]
#[feature(unwrap_infallible)]
// tidy-alphabetical-end

use std::fmt;

pub use atomic_ref::AtomicRef;
pub use ena::{snapshot_vec, undo_log, unify};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=outline | COMPLEXITY=5 | LINES=48 */
pub use crate::rustc_index::static_assert_size;



/// This calls the passed function while ensuring it won't be inlined into the caller.
#[inline(never)]
#[cold]
pub fn outline<F: FnOnce() -> R, R>(f: F) -> R {
    f()
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=defer | COMPLEXITY=2 | LINES=5 */

/// Returns a structure that calls `f` when dropped.
pub fn defer<F: FnOnce()>(f: F) -> OnDrop<F> {
    OnDrop(Some(f))
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=OnDrop | COMPLEXITY=3 | LINES=10 */

pub struct OnDrop<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> OnDrop<F> {
    /// Disables on-drop call.
    #[inline]
    pub fn disable(mut self) {
        self.0.take();
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=drop | COMPLEXITY=8 | LINES=9 */

impl<F: FnOnce()> Drop for OnDrop<F> {
    #[inline]
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=FatalErrorMarker; | COMPLEXITY=11 | LINES=20 */

/// This is a marker for a fatal compiler error used with `resume_unwind`.
pub struct FatalErrorMarker;

/// Turns a closure that takes an `&mut Formatter` into something that can be display-formatted.
pub fn make_display(f: impl Fn(&mut fmt::Formatter<'_>) -> fmt::Result) -> impl fmt::Display {
    struct Printer<F> {
        f: F,
    }
    impl<F> fmt::Display for Printer<F>
    where
        F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
    {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            (self.f)(fmt)
        }
    }

    Printer { f }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=__noop_fix_for_windows_dllimport_issue | COMPLEXITY=2 | LINES=4 */

// See comment in compiler/rustc_middle/src/tests.rs and issue #27438.
#[doc(hidden)]
pub fn __noop_fix_for_windows_dllimport_issue() {}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=12 | LINES=11 */

#[macro_export]
macro_rules! external_bitflags_debug {
    ($Name:ident) => {
        impl ::std::fmt::Debug for $Name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::bitflags::parser::to_writer(self, f)
            }
        }
    };
}