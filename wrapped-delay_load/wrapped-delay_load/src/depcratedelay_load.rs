// Generated macro for delay_load (function)
macro_rules! Depcratedelay_load {
() => {
// Module: crate
// Provides: {"delay_load"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " The `PCSTR` parameters need to be valid for reads up until and including the next `\\0`."] pub unsafe fn delay_load < T > (library : PCSTR , function : PCSTR) -> Option < T > { unsafe { let library = LoadLibraryExA (library , None , LOAD_LIBRARY_SEARCH_DEFAULT_DIRS) ; let Ok (library) = library else { return None ; } ; let address = GetProcAddress (library , function) ; if address . is_some () { return Some (std :: mem :: transmute_copy (& address)) ; } _ = FreeLibrary (library) ; None } }
};
}
