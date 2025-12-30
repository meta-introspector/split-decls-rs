// Generated macro for select_api_key (function)
macro_rules! Depcrate_isa_aarch64_abiselect_api_key {
() => {
// Module: crate::isa::aarch64::abi
// Provides: {"select_api_key"}
// Dependencies: {}
fn select_api_key (isa_flags : & aarch64_settings :: Flags , call_conv : isa :: CallConv , setup_frame : bool ,) -> Option < APIKey > { if isa_flags . sign_return_address () && (setup_frame || isa_flags . sign_return_address_all ()) { Some (if isa_flags . sign_return_address_with_bkey () { match call_conv { isa :: CallConv :: Tail => APIKey :: BZ , _ => APIKey :: BSP , } } else { match call_conv { isa :: CallConv :: Tail => APIKey :: AZ , _ => APIKey :: ASP , } }) } else { None } }
};
}
