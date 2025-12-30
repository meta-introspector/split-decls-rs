// Generated macro for mockable_foreign_mod (function)
macro_rules! Depcrate_mockable_itemmockable_foreign_mod {
() => {
// Module: crate::mockable_item
// Provides: {"mockable_foreign_mod"}
// Dependencies: {}
# [doc = " Performs transformations on a Foreign Mod to make it mockable"] fn mockable_foreign_mod (mut ifm : ItemForeignMod) -> ItemForeignMod { for item in & mut ifm . items { if let ForeignItem :: Fn (ref mut f) = item { fix_elipses (& mut f . sig) ; } } ifm }
};
}
