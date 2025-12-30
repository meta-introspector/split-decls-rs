// Generated macro for impl_116 (impl)
macro_rules! Depcrate_utilsimpl_116 {
() => {
// Module: crate::utils
// Provides: {"impl_116"}
// Dependencies: {}
impl Color { # [inline] fn ansi_num (self) -> usize { match self { Color :: Black => 0 , Color :: Red => 1 , Color :: Green => 2 , Color :: Yellow => 3 , Color :: Blue => 4 , Color :: Magenta => 5 , Color :: Cyan => 6 , Color :: White => 7 , Color :: Color256 (x) => x as usize , Color :: TrueColor (_ , _ , _) => panic ! ("RGB colors must be handled separately") , } } # [inline] fn is_color256 (self) -> bool { # [allow (clippy :: match_like_matches_macro)] match self { Color :: Color256 (_) => true , _ => false , } } }
};
}
