// Generated macro for impl_7 (impl)
macro_rules! Depcrate_gif_supportimpl_7 {
() => {
// Module: crate::gif_support
// Provides: {"impl_7"}
// Dependencies: {}
impl GifFile { pub (super) fn new < T : AsRef < Path > > (path : T , dim : (u32 , u32) , delay : u32 ,) -> Result < Self , BitMapBackendError > { let mut encoder = GifEncoder :: new (File :: create (path . as_ref ()) . map_err (BitMapBackendError :: IOError) ? , dim . 0 as u16 , dim . 1 as u16 , & [] ,) . map_err (BitMapBackendError :: GifEncodingError) ? ; encoder . set_repeat (Repeat :: Infinite) . map_err (BitMapBackendError :: GifEncodingError) ? ; Ok (Self { encoder , width : dim . 0 , height : dim . 1 , delay : (delay + 5) / 10 , }) } pub (super) fn flush_frame (& mut self , buffer : & [u8]) -> Result < () , BitMapBackendError > { let mut frame = GifFrame :: from_rgb_speed (self . width as u16 , self . height as u16 , buffer , 10) ; frame . delay = self . delay as u16 ; self . encoder . write_frame (& frame) . map_err (BitMapBackendError :: GifEncodingError) ? ; Ok (()) } }
};
}
