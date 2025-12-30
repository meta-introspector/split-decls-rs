// Generated macro for get_dxgi_factory (function)
macro_rules! Depcrateget_dxgi_factory {
() => {
// Module: crate
// Provides: {"get_dxgi_factory"}
// Dependencies: {}
fn get_dxgi_factory (device : & ID3D11Device) -> Result < IDXGIFactory2 > { let dxdevice = device . cast :: < IDXGIDevice > () ? ; unsafe { dxdevice . GetAdapter () ? . GetParent () } }
};
}
