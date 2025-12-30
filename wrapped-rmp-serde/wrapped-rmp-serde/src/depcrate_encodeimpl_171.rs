// Generated macro for impl_171 (impl)
macro_rules! Depcrate_encodeimpl_171 {
() => {
// Module: crate::encode
// Provides: {"impl_171"}
// Dependencies: {}
impl < W : Write , C : SerializerConfig > Serializer < W , C > { fn bytes_from_iter < I > (& mut self , mut iter : I , len : u32) -> Result < () , < & mut Self as serde :: Serializer > :: Error > where I : Iterator , I :: Item : Serialize { encode :: write_bin_len (& mut self . wr , len) ? ; iter . try_for_each (| item | { self . wr . write (std :: slice :: from_ref (& item . serialize (OnlyBytes) . map_err (| _ | Error :: InvalidDataModel ("BytesMode")) ?)) . map_err (ValueWriteError :: InvalidDataWrite) ? ; Ok (()) }) } }
};
}
