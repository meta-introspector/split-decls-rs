// Generated macro for impl_8006 (impl)
macro_rules! Depcrate_non_copy_constimpl_8006 {
() => {
// Module: crate::non_copy_const
// Provides: {"impl_8006"}
// Dependencies: {}
impl IsFreeze { # [doc = " Merges the variants of a sum type (i.e. an enum)."] fn from_variants (iter : impl Iterator < Item = Self >) -> Self { iter . fold (Self :: Yes , | x , y | match (x , y) { (Self :: Maybe , _) | (_ , Self :: Maybe) | (Self :: No , Self :: Yes) | (Self :: Yes , Self :: No) => Self :: Maybe , (Self :: No , Self :: No) => Self :: No , (Self :: Yes , Self :: Yes) => Self :: Yes , }) } # [doc = " Merges the fields of a product type (e.g. a struct or tuple)."] fn from_fields (mut iter : impl Iterator < Item = Self >) -> Self { iter . try_fold (Self :: Yes , | x , y | match (x , y) { (Self :: No , _) | (_ , Self :: No) => None , (Self :: Maybe , _) | (_ , Self :: Maybe) => Some (Self :: Maybe) , (Self :: Yes , Self :: Yes) => Some (Self :: Yes) , }) . unwrap_or (Self :: No) } # [doc = " Checks if this is definitely `Freeze`."] fn is_freeze (self) -> bool { matches ! (self , Self :: Yes) } # [doc = " Checks if this is definitely not `Freeze`."] fn is_not_freeze (self) -> bool { matches ! (self , Self :: No) } }
};
}
