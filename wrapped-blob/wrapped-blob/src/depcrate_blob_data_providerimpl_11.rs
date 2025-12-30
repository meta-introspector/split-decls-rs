// Generated macro for impl_11 (impl)
macro_rules! Depcrate_blob_data_providerimpl_11 {
() => {
// Module: crate::blob_data_provider
// Provides: {"impl_11"}
// Dependencies: {}
impl BlobDataProvider { # [doc = " Create a [`BlobDataProvider`] from a blob of ICU4X data."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] pub fn try_new_from_blob (blob : alloc :: boxed :: Box < [u8] >) -> Result < Self , DataError > { Ok (Self { data : Cart :: try_make_yoke (blob , | bytes | { BlobSchema :: deserialize_and_check (& mut postcard :: Deserializer :: from_bytes (bytes)) }) ? , }) } # [doc = " Create a [`BlobDataProvider`] from a static blob. This is a special case of"] # [doc = " [`try_new_from_blob`](BlobDataProvider::try_new_from_blob) and is allocation-free."] pub fn try_new_from_static_blob (blob : & 'static [u8]) -> Result < Self , DataError > { Ok (Self { data : Yoke :: new_owned (BlobSchema :: deserialize_and_check (& mut postcard :: Deserializer :: from_bytes (blob) ,) ?) , }) } # [doc (hidden)] pub fn internal_is_using_bigger_format (& self) -> bool { matches ! (self . data . get () , BlobSchema :: V003Bigger (..)) } }
};
}
