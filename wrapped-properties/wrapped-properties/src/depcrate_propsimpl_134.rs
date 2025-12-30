// Generated macro for impl_134 (impl)
macro_rules! Depcrate_propsimpl_134 {
() => {
// Module: crate::props
// Provides: {"impl_134"}
// Dependencies: {}
impl TryFrom < u8 > for GeneralCategory { type Error = GeneralCategoryOutOfBoundsError ; # [doc = " Construct this [`GeneralCategory`] from an integer, returning"] # [doc = " an error if it is out of bounds"] fn try_from (val : u8) -> Result < Self , GeneralCategoryOutOfBoundsError > { GeneralCategory :: new_from_u8 (val) . ok_or (GeneralCategoryOutOfBoundsError) } }
};
}
