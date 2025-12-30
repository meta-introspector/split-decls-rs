// Generated macro for impl_679 (impl)
macro_rules! Depcrate_writerimpl_679 {
() => {
// Module: crate::writer
// Provides: {"impl_679"}
// Dependencies: {}
# [cfg (feature = "serialize")] impl < T > std :: fmt :: Write for ToFmtWrite < T > where T : std :: io :: Write , { fn write_str (& mut self , s : & str) -> std :: fmt :: Result { self . 0 . write_all (s . as_bytes ()) . map_err (| _ | std :: fmt :: Error) } }
};
}
