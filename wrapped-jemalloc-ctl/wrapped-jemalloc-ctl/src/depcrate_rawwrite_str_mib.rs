// Generated macro for write_str_mib (function)
macro_rules! Depcrate_rawwrite_str_mib {
() => {
// Module: crate::raw
// Provides: {"write_str_mib"}
// Dependencies: {}
# [doc = " Uses the MIB `mib` as key to the _MALLCTL NAMESPACE_ and writes its `value`."] # [doc = ""] # [doc = " The [`name_to_mib`] API translates a string of the key (e.g. `arenas.nbins`)"] # [doc = " to a `mib` (Management Information Base)."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If `value` is not a non-empty null-terminated string."] pub fn write_str_mib (mib : & [usize] , value : & 'static [u8]) -> Result < () > { assert ! (! value . is_empty () , "value cannot be empty") ; assert_eq ! (* value . last () . unwrap () , b'\0') ; unsafe { write_mib (mib , value . as_ptr () as * const c_char) } }
};
}
