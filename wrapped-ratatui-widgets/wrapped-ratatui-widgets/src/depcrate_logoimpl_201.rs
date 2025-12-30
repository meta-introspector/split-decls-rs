// Generated macro for impl_201 (impl)
macro_rules! Depcrate_logoimpl_201 {
() => {
// Module: crate::logo
// Provides: {"impl_201"}
// Dependencies: {}
impl RatatuiLogo { # [doc = " Create a new Ratatui logo widget"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ratatui::widgets::{RatatuiLogo, RatatuiLogoSize};"] # [doc = ""] # [doc = " let logo = RatatuiLogo::new(RatatuiLogoSize::Tiny);"] # [doc = " ```"] pub const fn new (size : Size) -> Self { Self { size } } # [doc = " Set the size of the logo"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ratatui::widgets::{RatatuiLogo, RatatuiLogoSize};"] # [doc = ""] # [doc = " let logo = RatatuiLogo::default().size(RatatuiLogoSize::Small);"] # [doc = " ```"] # [must_use] pub const fn size (self , size : Size) -> Self { let _ = self ; Self { size } } # [doc = " Create a new Ratatui logo widget with a tiny size"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ratatui::widgets::RatatuiLogo;"] # [doc = ""] # [doc = " let logo = RatatuiLogo::tiny();"] # [doc = " ```"] pub const fn tiny () -> Self { Self :: new (Size :: Tiny) } # [doc = " Create a new Ratatui logo widget with a small size"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ratatui::widgets::RatatuiLogo;"] # [doc = ""] # [doc = " let logo = RatatuiLogo::small();"] # [doc = " ```"] pub const fn small () -> Self { Self :: new (Size :: Small) } }
};
}
