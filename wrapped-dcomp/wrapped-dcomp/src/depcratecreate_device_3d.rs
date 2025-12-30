// Generated macro for create_device_3d (function)
macro_rules! Depcratecreate_device_3d {
() => {
// Module: crate
// Provides: {"create_device_3d"}
// Dependencies: {}
fn create_device_3d () -> Result < ID3D11Device > { let mut device = None ; unsafe { D3D11CreateDevice (None , D3D_DRIVER_TYPE_HARDWARE , HMODULE :: default () , D3D11_CREATE_DEVICE_BGRA_SUPPORT , None , D3D11_SDK_VERSION , Some (& mut device) , None , None ,) . map (| () | device . unwrap ()) } }
};
}
