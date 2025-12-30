// Generated macro for LmsModeInternal (struct)
macro_rules! Depcrate_lms_modesLmsModeInternal {
() => {
// Module: crate::lms::modes
// Provides: {"LmsModeInternal"}
// Dependencies: {}
# [derive (Debug)] pub struct LmsModeInternal < OtsMode : LmsOtsMode , Hasher : Digest , HLen : ArraySize , const M : usize , const H : usize , const TC : u32 , > { _ots_mode : PhantomData < OtsMode > , _hasher : PhantomData < Hasher > , _h_len : PhantomData < HLen > , }
};
}
