// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl < W > StrWrite for & '_ mut W where W : StrWrite , { type Error = W :: Error ; # [inline] fn write_str (& mut self , s : & str) -> Result < () , Self :: Error > { (* * self) . write_str (s) } # [inline] fn write_fmt (& mut self , args : Arguments) -> Result < () , Self :: Error > { (* * self) . write_fmt (args) } }
};
}
