// Generated macro for io (module)
macro_rules! Depcrate_ser_flavorsio {
() => {
// Module: crate::ser::flavors
// Provides: {"io"}
// Dependencies: {}
# [doc = " Support for the [`std::io`] traits"] # [cfg (feature = "use-std")] pub mod io { use super :: Flavor ; use crate :: { Error , Result } ; # [doc = " Wrapper over a [`std::io::Write`] that implements the flavor trait"] pub struct WriteFlavor < T > { writer : T , } impl < T > WriteFlavor < T > where T : std :: io :: Write , { # [doc = " Create a new [`Self`] flavor from a given [`std::io::Write`]"] pub fn new (writer : T) -> Self { Self { writer } } } impl < T > Flavor for WriteFlavor < T > where T : std :: io :: Write , { type Output = T ; # [inline (always)] fn try_push (& mut self , data : u8) -> Result < () > { self . writer . write_all (& [data]) . map_err (| _ | Error :: SerializeBufferFull) ? ; Ok (()) } # [inline (always)] fn try_extend (& mut self , b : & [u8]) -> Result < () > { self . writer . write_all (b) . map_err (| _ | Error :: SerializeBufferFull) ? ; Ok (()) } fn finalize (mut self) -> Result < Self :: Output > { self . writer . flush () . map_err (| _ | Error :: SerializeBufferFull) ? ; Ok (self . writer) } } }
};
}
