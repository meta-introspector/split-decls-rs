// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
# [cfg (feature = "std")] impl < W > StrWrite for IoWriter < W > where W : Write , { type Error = io :: Error ; # [inline] fn write_str (& mut self , s : & str) -> io :: Result < () > { self . 0 . write_all (s . as_bytes ()) } # [inline] fn write_fmt (& mut self , args : Arguments) -> io :: Result < () > { self . 0 . write_fmt (args) } }
};
}
