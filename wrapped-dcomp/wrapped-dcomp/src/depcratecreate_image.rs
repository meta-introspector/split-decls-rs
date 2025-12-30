// Generated macro for create_image (function)
macro_rules! Depcratecreate_image {
() => {
// Module: crate
// Provides: {"create_image"}
// Dependencies: {}
fn create_image () -> Result < IWICFormatConverter > { unsafe { let factory : IWICImagingFactory2 = CoCreateInstance (& CLSID_WICImagingFactory , None , CLSCTX_INPROC_SERVER) ? ; let path = if PathFileExistsW (w ! ("image.jpg")) . is_ok () { w ! ("image.jpg") } else { w ! ("crates/samples/windows/dcomp/image.jpg") } ; let decoder = factory . CreateDecoderFromFilename (path , None , GENERIC_READ , WICDecodeMetadataCacheOnDemand ,) ? ; let source = decoder . GetFrame (0) ? ; let image = factory . CreateFormatConverter () ? ; image . Initialize (& source , & GUID_WICPixelFormat32bppBGR , WICBitmapDitherTypeNone , None , 0.0 , WICBitmapPaletteTypeMedianCut ,) ? ; Ok (image) } }
};
}
