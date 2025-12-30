// Generated macro for other_204 (other)
macro_rules! Depcrate_font_managerother_204 {
() => {
// Module: crate::font_manager
// Provides: {"other_204"}
// Dependencies: {}
extern "C" { pub fn CTFontManagerCopyAvailableFontURLs () -> CFArrayRef ; pub fn CTFontManagerCopyAvailableFontFamilyNames () -> CFArrayRef ; pub fn CTFontManagerCopyAvailablePostScriptNames () -> CFArrayRef ; pub fn CTFontManagerCreateFontDescriptorsFromURL (fileURL : CFURLRef) -> CFArrayRef ; pub fn CTFontManagerCreateFontDescriptorFromData (data : CFDataRef) -> CTFontDescriptorRef ; pub fn CTFontManagerIsSupportedFont (fontURL : CFURLRef) -> bool ; }
};
}
