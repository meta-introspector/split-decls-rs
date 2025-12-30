// Generated macro for create_render_target (function)
macro_rules! Depcratecreate_render_target {
() => {
// Module: crate
// Provides: {"create_render_target"}
// Dependencies: {}
fn create_render_target (factory : & ID2D1Factory1 , device : & ID3D11Device ,) -> Result < ID2D1DeviceContext > { unsafe { let d2device = factory . CreateDevice (& device . cast :: < IDXGIDevice > () ?) ? ; let target = d2device . CreateDeviceContext (D2D1_DEVICE_CONTEXT_OPTIONS_NONE) ? ; target . SetUnitMode (D2D1_UNIT_MODE_DIPS) ; Ok (target) } }
};
}
