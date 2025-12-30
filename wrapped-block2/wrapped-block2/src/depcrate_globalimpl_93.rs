// Generated macro for impl_93 (impl)
macro_rules! Depcrate_globalimpl_93 {
() => {
// Module: crate::global
// Provides: {"impl_93"}
// Dependencies: {}
impl < F : ? Sized > GlobalBlock < F > { const FLAGS : BlockFlags = BlockFlags :: BLOCK_IS_GLOBAL . union (BlockFlags :: BLOCK_USE_STRET) ; # [doc (hidden)] # [allow (clippy :: declare_interior_mutable_const)] pub const __DEFAULT_HEADER : BlockHeader = BlockHeader { isa : ptr :: null_mut () , flags : Self :: FLAGS , reserved : MaybeUninit :: new (0) , invoke : None , descriptor : BlockDescriptorPtr { basic : & GLOBAL_DESCRIPTOR , } , } ; # [doc = " Use the [`global_block`] macro instead."] # [doc (hidden)] # [inline] pub const unsafe fn from_header (header : BlockHeader) -> Self { Self { header , f : PhantomData , } } }
};
}
