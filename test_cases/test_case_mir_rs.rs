// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_public/src/unstable/convert/stable/mir.rs
// Error: expected square brackets
// Problematic line: line 15

use crate::unstable::Stable;
use crate::{Error, alloc, opaque};

impl<'tcx> Stable<'tcx> for mir::Body<'tcx> {
    type T = crate::mir::Body;

    fn stable<'cx>(
