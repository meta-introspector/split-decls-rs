// Generated macro for create_swapchain (function)
macro_rules! Depcratecreate_swapchain {
() => {
// Module: crate
// Provides: {"create_swapchain"}
// Dependencies: {}
fn create_swapchain (device : & ID3D11Device , window : HWND) -> Result < IDXGISwapChain1 > { let factory = get_dxgi_factory (device) ? ; let props = DXGI_SWAP_CHAIN_DESC1 { Format : DXGI_FORMAT_B8G8R8A8_UNORM , SampleDesc : DXGI_SAMPLE_DESC { Count : 1 , Quality : 0 , } , BufferUsage : DXGI_USAGE_RENDER_TARGET_OUTPUT , BufferCount : 2 , SwapEffect : DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL , .. Default :: default () } ; unsafe { factory . CreateSwapChainForHwnd (device , window , & props , None , None) } }
};
}
