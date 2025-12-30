// Generated macro for impl_33 (impl)
macro_rules! Depcrate_fs_dirimpl_33 {
() => {
// Module: crate::fs::dir
// Provides: {"impl_33"}
// Dependencies: {}
impl DotFilter { # [doc = " Whether this filter should show dotfiles in a listing."] fn shows_dotfiles (self) -> bool { match self { Self :: JustFiles => false , Self :: Dotfiles => true , Self :: DotfilesAndDots => true , } } # [doc = " Whether this filter should add dot directories to a listing."] fn dots (self) -> DotsNext { match self { Self :: JustFiles => DotsNext :: Files , Self :: Dotfiles => DotsNext :: Files , Self :: DotfilesAndDots => DotsNext :: Dot , } } }
};
}
