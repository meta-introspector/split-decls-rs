// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
impl < T : AsRef < str > > Casing < T > for T { fn to_case (& self , case : Case) -> String { StateConverter :: new (self) . to_case (case) } fn with_boundaries (& self , bs : & [Boundary]) -> StateConverter < '_ , T > { StateConverter :: new (self) . set_boundaries (bs) } fn without_boundaries (& self , bs : & [Boundary]) -> StateConverter < '_ , T > { StateConverter :: new (self) . without_boundaries (bs) } fn from_case (& self , case : Case) -> StateConverter < '_ , T > { StateConverter :: new (self) . from_case (case) } fn is_case (& self , case : Case) -> bool { self . as_ref () == self . to_case (case) . as_str () } }
};
}
