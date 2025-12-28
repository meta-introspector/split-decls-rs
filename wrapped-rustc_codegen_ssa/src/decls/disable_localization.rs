macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! disable_localization {
    () => {
        deps!();
        # [doc = " Disables non-English messages from localized linkers."] # [doc = " Such messages may cause issues with text encoding on Windows (#35785)"] # [doc = " and prevent inspection of linker output in case of errors, which we occasionally do."] # [doc = " This should be acceptable because other messages from rustc are in English anyway,"] # [doc = " and may also be desirable to improve searchability of the linker diagnostics."] pub (crate) fn disable_localization (linker : & mut Command) { linker . env ("LC_ALL" , "C") ; linker . env ("VSLANG" , "1033") ; }
    };
}

disable_localization!();