// Generated macro for impl_1736 (impl)
macro_rules! Depcrate_isa_aarch64_inst_argsimpl_1736 {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"impl_1736"}
// Dependencies: {}
impl Cond { # [doc = " Return the inverted condition."] pub fn invert (self) -> Cond { match self { Cond :: Eq => Cond :: Ne , Cond :: Ne => Cond :: Eq , Cond :: Hs => Cond :: Lo , Cond :: Lo => Cond :: Hs , Cond :: Mi => Cond :: Pl , Cond :: Pl => Cond :: Mi , Cond :: Vs => Cond :: Vc , Cond :: Vc => Cond :: Vs , Cond :: Hi => Cond :: Ls , Cond :: Ls => Cond :: Hi , Cond :: Ge => Cond :: Lt , Cond :: Lt => Cond :: Ge , Cond :: Gt => Cond :: Le , Cond :: Le => Cond :: Gt , Cond :: Al => Cond :: Nv , Cond :: Nv => Cond :: Al , } } # [doc = " Return the machine encoding of this condition."] pub fn bits (self) -> u32 { self as u32 } }
};
}
