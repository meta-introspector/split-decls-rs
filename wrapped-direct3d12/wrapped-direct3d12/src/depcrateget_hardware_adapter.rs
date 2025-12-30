// Generated macro for get_hardware_adapter (function)
macro_rules! Depcrateget_hardware_adapter {
() => {
// Module: crate
// Provides: {"get_hardware_adapter"}
// Dependencies: {}
fn get_hardware_adapter (factory : & IDXGIFactory4) -> Result < IDXGIAdapter1 > { for i in 0 .. { let adapter = unsafe { factory . EnumAdapters1 (i) ? } ; let desc = unsafe { adapter . GetDesc1 () ? } ; if (DXGI_ADAPTER_FLAG (desc . Flags as _) & DXGI_ADAPTER_FLAG_SOFTWARE) != DXGI_ADAPTER_FLAG_NONE { continue ; } if unsafe { D3D12CreateDevice (& adapter , D3D_FEATURE_LEVEL_11_0 , std :: ptr :: null_mut :: < Option < ID3D12Device > > () ,) } . is_ok () { return Ok (adapter) ; } } unreachable ! () }
};
}
