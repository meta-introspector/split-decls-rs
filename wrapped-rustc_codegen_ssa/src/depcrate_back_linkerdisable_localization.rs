// Generated macro for disable_localization (function)
macro_rules! Depcrate_back_linkerdisable_localization {
() => {
// Module: crate::back::linker
// Provides: {"disable_localization"}
// Dependencies: {}
# [doc = " Disables non-English messages from localized linkers."] # [doc = " Such messages may cause issues with text encoding on Windows (#35785)"] # [doc = " and prevent inspection of linker output in case of errors, which we occasionally do."] # [doc = " This should be acceptable because other messages from rustc are in English anyway,"] # [doc = " and may also be desirable to improve searchability of the linker diagnostics."] pub (crate) fn disable_localization (linker : & mut Command) { linker . env ("LC_ALL" , "C") ; linker . env ("VSLANG" , "1033") ; }
};
}
