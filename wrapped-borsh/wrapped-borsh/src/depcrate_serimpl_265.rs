// Generated macro for impl_265 (impl)
macro_rules! Depcrate_serimpl_265 {
() => {
// Module: crate::ser
// Provides: {"impl_265"}
// Dependencies: {}
impl < T , E > BorshSerialize for core :: result :: Result < T , E > where T : BorshSerialize , E : BorshSerialize , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { match self { Err (e) => { 0u8 . serialize (writer) ? ; e . serialize (writer) } Ok (v) => { 1u8 . serialize (writer) ? ; v . serialize (writer) } } } }
};
}
