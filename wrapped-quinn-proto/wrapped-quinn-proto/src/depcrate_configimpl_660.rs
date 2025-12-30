// Generated macro for impl_660 (impl)
macro_rules! Depcrate_configimpl_660 {
() => {
// Module: crate::config
// Provides: {"impl_660"}
// Dependencies: {}
impl fmt :: Debug for ServerConfig { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("ServerConfig") . field ("transport" , & self . transport) . field ("retry_token_lifetime" , & self . retry_token_lifetime) . field ("validation_token" , & self . validation_token) . field ("migration" , & self . migration) . field ("preferred_address_v4" , & self . preferred_address_v4) . field ("preferred_address_v6" , & self . preferred_address_v6) . field ("max_incoming" , & self . max_incoming) . field ("incoming_buffer_size" , & self . incoming_buffer_size) . field ("incoming_buffer_size_total" , & self . incoming_buffer_size_total ,) . finish_non_exhaustive () } }
};
}
