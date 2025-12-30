// Generated macro for macro_418 (macro)
macro_rules! Depcrate_arbitrary__std_ffimacro_418 {
() => {
// Module: crate::arbitrary::_std::ffi
// Provides: {"macro_418"}
// Dependencies: {}
arbitrary ! (FromBytesWithNulError , SMapped < Option < u16 >, Self >; { static_map (any ::< Option < u16 >> () , | opt_pos | { if let Some (pos) = opt_pos { let pos = pos as usize ; let mut v = Vec ::< u8 >:: with_capacity (pos + 2) ; v . extend (:: std :: iter :: repeat (1) . take (pos)) ; v . push (0) ; v . push (1) ; CStr :: from_bytes_with_nul (v . as_slice ()) . unwrap_err () } else { CStr :: from_bytes_with_nul (b"") . unwrap_err () } }) }) ;
};
}
