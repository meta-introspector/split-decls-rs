// Generated macro for NSVisualEffectMaterial (enum)
macro_rules! Depcrate_appkitNSVisualEffectMaterial {
() => {
// Module: crate::appkit
// Provides: {"NSVisualEffectMaterial"}
// Dependencies: {}
# [doc = " <https://developer.apple.com/documentation/appkit/nsvisualeffectview/material>"] # [repr (u64)] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum NSVisualEffectMaterial { # [doc = " A default material for the view's effectiveAppearance."] # [deprecated = "Use a semantic material instead."] AppearanceBased = 0 , # [deprecated = "Use a semantic material instead."] Light = 1 , # [deprecated = "Use a semantic material instead."] Dark = 2 , # [deprecated = "Use a semantic material instead."] MediumLight = 8 , # [deprecated = "Use a semantic material instead."] UltraDark = 9 , Titlebar = 3 , Selection = 4 , Menu = 5 , Popover = 6 , Sidebar = 7 , HeaderView = 10 , Sheet = 11 , WindowBackground = 12 , HudWindow = 13 , FullScreenUI = 15 , Tooltip = 17 , ContentBackground = 18 , UnderWindowBackground = 21 , UnderPageBackground = 22 , }
};
}
