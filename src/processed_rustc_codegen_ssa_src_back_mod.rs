// SRC: ../rust/compiler/rustc_codegen_ssa/src/back/mod.rs
use std::borrow::Cow;

use crate::rustc_complete::Session;


/// The target triple depends on the deployment target, and is required to
/// enable features such as cross-language LTO, and for picking the right
/// Mach-O commands.
///
/// Certain optimizations also depend on the deployment target.
pub fn versioned_llvm_target(sess: &Session) -> Cow<'_, str> {
    if sess.target.is_like_darwin {
        apple::add_version_to_llvm_target(&sess.target.llvm_target, sess.apple_deployment_target())
            .into()
    } else {
        // FIXME(madsmtm): Certain other targets also include a version,
        // we might want to move that here as well.
        Cow::Borrowed(&sess.target.llvm_target)
    }
}