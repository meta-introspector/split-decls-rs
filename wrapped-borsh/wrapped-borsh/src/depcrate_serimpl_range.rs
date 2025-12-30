// Generated macro for impl_range (macro)
macro_rules! Depcrate_serimpl_range {
() => {
// Module: crate::ser
// Provides: {"impl_range"}
// Dependencies: {}
macro_rules ! impl_range { ($ type : ident , $ this : ident , $ ($ field : expr) ,*) => { impl < T : BorshSerialize > BorshSerialize for core :: ops ::$ type < T > { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { let $ this = self ; $ ($ field . serialize (writer) ?;) * Ok (()) } } } ; }
};
}
