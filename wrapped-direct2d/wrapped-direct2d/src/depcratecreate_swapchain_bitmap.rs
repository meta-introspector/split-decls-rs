// Generated macro for create_swapchain_bitmap (function)
macro_rules! Depcratecreate_swapchain_bitmap {
() => {
// Module: crate
// Provides: {"create_swapchain_bitmap"}
// Dependencies: {}
fn create_swapchain_bitmap (swapchain : & IDXGISwapChain1 , target : & ID2D1DeviceContext) -> Result < () > { let surface : IDXGISurface = unsafe { swapchain . GetBuffer (0) ? } ; let props = D2D1_BITMAP_PROPERTIES1 { pixelFormat : D2D1_PIXEL_FORMAT { format : DXGI_FORMAT_B8G8R8A8_UNORM , alphaMode : D2D1_ALPHA_MODE_IGNORE , } , dpiX : 96.0 , dpiY : 96.0 , bitmapOptions : D2D1_BITMAP_OPTIONS_TARGET | D2D1_BITMAP_OPTIONS_CANNOT_DRAW , .. Default :: default () } ; unsafe { let bitmap = target . CreateBitmapFromDxgiSurface (& surface , Some (& props)) ? ; target . SetTarget (& bitmap) ; } ; Ok (()) }
};
}
