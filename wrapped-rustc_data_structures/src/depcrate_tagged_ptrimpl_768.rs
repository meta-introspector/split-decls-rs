// Generated macro for impl_768 (impl)
macro_rules! Depcrate_tagged_ptrimpl_768 {
() => {
// Module: crate::tagged_ptr
// Provides: {"impl_768"}
// Dependencies: {}
impl < 'a , P , T > TaggedRef < 'a , P , T > where P : Aligned + ? Sized , T : Tag , { # [doc = " Tags `pointer` with `tag`."] # [doc = ""] # [doc = " [`TaggedRef`]: crate::tagged_ptr::TaggedRef"] # [inline] pub fn new (pointer : & 'a P , tag : T) -> Self { Self { packed : Self :: pack (NonNull :: from (pointer) , tag) , tag_pointer_ghost : PhantomData } } # [doc = " Retrieves the pointer."] # [inline] pub fn pointer (self) -> & 'a P { unsafe { self . pointer_raw () . as_ref () } } # [doc = " Retrieves the tag."] # [inline] pub fn tag (& self) -> T { let tag = self . packed . addr () . get () >> Self :: TAG_BIT_SHIFT ; unsafe { T :: from_usize (tag) } } # [doc = " Sets the tag to a new value."] # [inline] pub fn set_tag (& mut self , tag : T) { self . packed = Self :: pack (self . pointer_raw () , tag) ; } const TAG_BIT_SHIFT : u32 = usize :: BITS - T :: BITS ; const ASSERTION : () = { assert ! (T :: BITS <= bits_for ::< P > ()) } ; # [doc = " Pack pointer `ptr` with a `tag`, according to `self.packed` encoding scheme."] # [inline] fn pack (ptr : NonNull < P > , tag : T) -> NonNull < P > { let () = Self :: ASSERTION ; let packed_tag = tag . into_usize () << Self :: TAG_BIT_SHIFT ; ptr . map_addr (| addr | { let packed = (addr . get () >> T :: BITS) | packed_tag ; unsafe { NonZero :: new_unchecked (packed) } }) } # [doc = " Retrieves the original raw pointer from `self.packed`."] # [inline] pub (super) fn pointer_raw (& self) -> NonNull < P > { self . packed . map_addr (| addr | unsafe { NonZero :: new_unchecked (addr . get () << T :: BITS) }) } }
};
}
