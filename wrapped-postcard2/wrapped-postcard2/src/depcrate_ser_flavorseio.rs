// Generated macro for eio (module)
macro_rules! Depcrate_ser_flavorseio {
() => {
// Module: crate::ser::flavors
// Provides: {"eio"}
// Dependencies: {}
# [doc = " Support for the [`embedded-io`](crate::eio::embedded_io) traits"] # [cfg (any (feature = "embedded-io-04" , feature = "embedded-io-06"))] pub mod eio { use super :: Flavor ; use crate :: { Error , Result } ; # [doc = " Wrapper over a [`embedded_io Write`](crate::eio::Write) that implements the flavor trait"] pub struct WriteFlavor < T > { writer : T , } impl < T > WriteFlavor < T > where T : crate :: eio :: Write , { # [doc = " Create a new [`Self`] flavor from a given [`embedded_io Write`](crate::eio::Write)"] pub fn new (writer : T) -> Self { Self { writer } } } impl < T > Flavor for WriteFlavor < T > where T : crate :: eio :: Write , { type Output = T ; # [inline (always)] fn try_push (& mut self , data : u8) -> Result < () > { self . writer . write_all (& [data]) . map_err (| _ | Error :: SerializeBufferFull) ? ; Ok (()) } # [inline (always)] fn try_extend (& mut self , b : & [u8]) -> Result < () > { self . writer . write_all (b) . map_err (| _ | Error :: SerializeBufferFull) ? ; Ok (()) } fn finalize (mut self) -> Result < Self :: Output > { self . writer . flush () . map_err (| _ | Error :: SerializeBufferFull) ? ; Ok (self . writer) } } }
};
}
