// Generated macro for create_device_2d (function)
macro_rules! Depcratecreate_device_2d {
() => {
// Module: crate
// Provides: {"create_device_2d"}
// Dependencies: {}
fn create_device_2d (device_3d : & ID3D11Device) -> Result < ID2D1Device > { let dxgi : IDXGIDevice3 = device_3d . cast () ? ; unsafe { D2D1CreateDevice (& dxgi , None) } }
};
}
