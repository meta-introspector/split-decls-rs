// Generated macro for BitMapBackend (struct)
macro_rules! Depcrate_bitmapBitMapBackend {
() => {
// Module: crate::bitmap
// Provides: {"BitMapBackend"}
// Dependencies: {}
# [doc = " The backend that drawing a bitmap"] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " You should call [`.present()?`](plotters_backend::DrawingBackend::present) on a"] # [doc = " `BitMapBackend`, not just `drop` it or allow it to go out of scope."] # [doc = ""] # [doc = " If the `BitMapBackend` is just dropped, it will make a best effort attempt to write the"] # [doc = " generated charts to the output file, but any errors that occur (such as inability to"] # [doc = " create the output file) will be silently ignored."] pub struct BitMapBackend < 'a , P : PixelFormat = RGBPixel > { # [doc = " The path to the image"] # [allow (dead_code)] target : Target < 'a > , # [doc = " The size of the image"] size : (u32 , u32) , # [doc = " The data buffer of the image"] buffer : Buffer < 'a > , # [doc = " Flag indicates if the bitmap has been saved"] saved : bool , _phantomdata : PhantomData < P > , }
};
}
