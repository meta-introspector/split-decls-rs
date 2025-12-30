// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl < BS : ArraySize , K : BufferKind > Default for BlockBuffer < BS , K > { # [inline] fn default () -> Self { assert ! (Self :: BLOCK_SIZE_ASSERT) ; let mut buffer = MaybeUninit :: uninit () ; let mut pos = Default :: default () ; K :: set_pos (& mut buffer , & mut pos , 0) ; Self { buffer , pos } } }
};
}
