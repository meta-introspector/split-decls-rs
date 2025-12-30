// Generated macro for InPlaceDstDataSrcBufDrop (struct)
macro_rules! Depcrate_vec_in_place_dropInPlaceDstDataSrcBufDrop {
() => {
// Module: crate::vec::in_place_drop
// Provides: {"InPlaceDstDataSrcBufDrop"}
// Dependencies: {}
pub (super) struct InPlaceDstDataSrcBufDrop < Src , Dest > { pub (super) ptr : NonNull < Dest > , pub (super) len : usize , pub (super) src_cap : usize , pub (super) src : PhantomData < Src > , }
};
}
