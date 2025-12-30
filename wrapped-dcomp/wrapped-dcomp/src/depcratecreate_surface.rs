// Generated macro for create_surface (function)
macro_rules! Depcratecreate_surface {
() => {
// Module: crate
// Provides: {"create_surface"}
// Dependencies: {}
fn create_surface (device : & IDCompositionDesktopDevice , width : f32 , height : f32 ,) -> Result < IDCompositionSurface > { unsafe { device . CreateSurface (width as u32 , height as u32 , DXGI_FORMAT_B8G8R8A8_UNORM , DXGI_ALPHA_MODE_PREMULTIPLIED ,) } }
};
}
