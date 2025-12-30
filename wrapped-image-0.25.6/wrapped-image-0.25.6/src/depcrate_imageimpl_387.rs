// Generated macro for impl_387 (impl)
macro_rules! Depcrate_imageimpl_387 {
() => {
// Module: crate::image
// Provides: {"impl_387"}
// Dependencies: {}
impl < T : ? Sized + ImageDecoder > ImageDecoder for Box < T > { fn dimensions (& self) -> (u32 , u32) { (* * self) . dimensions () } fn color_type (& self) -> ColorType { (* * self) . color_type () } fn original_color_type (& self) -> ExtendedColorType { (* * self) . original_color_type () } fn icc_profile (& mut self) -> ImageResult < Option < Vec < u8 > > > { (* * self) . icc_profile () } fn exif_metadata (& mut self) -> ImageResult < Option < Vec < u8 > > > { (* * self) . exif_metadata () } fn total_bytes (& self) -> u64 { (* * self) . total_bytes () } fn read_image (self , buf : & mut [u8]) -> ImageResult < () > where Self : Sized , { T :: read_image_boxed (self , buf) } fn read_image_boxed (self : Box < Self > , buf : & mut [u8]) -> ImageResult < () > { T :: read_image_boxed (* self , buf) } fn set_limits (& mut self , limits : crate :: Limits) -> ImageResult < () > { (* * self) . set_limits (limits) } }
};
}
