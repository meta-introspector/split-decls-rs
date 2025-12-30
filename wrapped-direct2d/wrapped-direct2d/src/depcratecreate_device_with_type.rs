// Generated macro for create_device_with_type (function)
macro_rules! Depcratecreate_device_with_type {
() => {
// Module: crate
// Provides: {"create_device_with_type"}
// Dependencies: {}
fn create_device_with_type (drive_type : D3D_DRIVER_TYPE) -> Result < ID3D11Device > { let mut flags = D3D11_CREATE_DEVICE_BGRA_SUPPORT ; if cfg ! (debug_assertions) { flags |= D3D11_CREATE_DEVICE_DEBUG ; } let mut device = None ; unsafe { D3D11CreateDevice (None , drive_type , HMODULE :: default () , flags , None , D3D11_SDK_VERSION , Some (& mut device) , None , None ,) . map (| () | device . unwrap ()) } }
};
}
