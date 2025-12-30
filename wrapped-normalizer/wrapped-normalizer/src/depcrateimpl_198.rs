// Generated macro for impl_198 (impl)
macro_rules! Depcrateimpl_198 {
() => {
// Module: crate
// Provides: {"impl_198"}
// Dependencies: {}
# [cfg (feature = "utf8_iter")] impl core :: fmt :: Write for IsNormalizedSinkUtf8 < '_ > { fn write_str (& mut self , s : & str) -> core :: fmt :: Result { # [expect (clippy :: indexing_slicing)] if core :: ptr :: eq (s . as_ptr () , self . expect . as_ptr ()) { self . expect = & self . expect [s . len () ..] ; Ok (()) } else { Err (core :: fmt :: Error { }) } } fn write_char (& mut self , c : char) -> core :: fmt :: Result { let mut iter = self . expect . chars () ; if iter . next () == Some (c) { self . expect = iter . as_slice () ; Ok (()) } else { Err (core :: fmt :: Error { }) } } }
};
}
