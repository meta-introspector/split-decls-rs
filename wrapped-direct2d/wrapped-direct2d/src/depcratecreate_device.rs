// Generated macro for create_device (function)
macro_rules! Depcratecreate_device {
() => {
// Module: crate
// Provides: {"create_device"}
// Dependencies: {}
fn create_device () -> Result < ID3D11Device > { let mut result = create_device_with_type (D3D_DRIVER_TYPE_HARDWARE) ; if let Err (err) = & result { if err . code () == DXGI_ERROR_UNSUPPORTED { result = create_device_with_type (D3D_DRIVER_TYPE_WARP) ; } } result }
};
}
