// Generated macro for impl_1500 (impl)
macro_rules! Depcrate_stringimpl_1500 {
() => {
// Module: crate::string
// Provides: {"impl_1500"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl fmt :: Write for String { # [inline] fn write_str (& mut self , s : & str) -> fmt :: Result { self . push_str (s) ; Ok (()) } # [inline] fn write_char (& mut self , c : char) -> fmt :: Result { self . push (c) ; Ok (()) } }
};
}
