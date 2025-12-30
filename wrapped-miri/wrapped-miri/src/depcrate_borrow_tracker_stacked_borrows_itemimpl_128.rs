// Generated macro for impl_128 (impl)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_itemimpl_128 {
() => {
// Module: crate::borrow_tracker::stacked_borrows::item
// Provides: {"impl_128"}
// Dependencies: {}
impl Item { pub fn new (tag : BorTag , perm : Permission , protected : bool) -> Self { assert ! (tag . get () <= TAG_MASK) ; let packed_tag = tag . get () ; let packed_perm = perm . to_bits () << PERM_SHIFT ; let packed_protected = u64 :: from (protected) << PROTECTED_SHIFT ; let new = Self (packed_tag | packed_perm | packed_protected) ; debug_assert ! (new . tag () == tag) ; debug_assert ! (new . perm () == perm) ; debug_assert ! (new . protected () == protected) ; new } # [doc = " The pointers the permission is granted to."] pub fn tag (self) -> BorTag { BorTag :: new (self . 0 & TAG_MASK) . unwrap () } # [doc = " The permission this item grants."] pub fn perm (self) -> Permission { Permission :: from_bits ((self . 0 & PERM_MASK) >> PERM_SHIFT) } # [doc = " Whether or not there is a protector for this tag"] pub fn protected (self) -> bool { self . 0 & PROTECTED_MASK > 0 } # [doc = " Set the Permission stored in this Item"] pub fn set_permission (& mut self , perm : Permission) { self . 0 &= ! PERM_MASK ; self . 0 |= perm . to_bits () << PERM_SHIFT ; } }
};
}
