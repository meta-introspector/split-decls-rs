// Generated macro for impl_98 (impl)
macro_rules! Depcrateimpl_98 {
() => {
// Module: crate
// Provides: {"impl_98"}
// Dependencies: {}
impl < CtxT > ConnectionConfig < CtxT > { # [doc = " Constructs the configuration required for a connection to be accepted."] pub fn new (context : CtxT) -> Self { Self { context , max_in_flight_operations : 0 , keep_alive_interval : Duration :: from_secs (15) , } } # [doc = " Specifies the maximum number of in-flight operations that a connection can have."] # [doc = ""] # [doc = " If this number is exceeded, attempting to start more will result in an error."] # [doc = " By default, there is no limit to in-flight operations."] # [must_use] pub fn with_max_in_flight_operations (mut self , max : usize) -> Self { self . max_in_flight_operations = max ; self } # [doc = " Specifies the interval at which to send keep-alives."] # [doc = ""] # [doc = " Specifying a [`Duration::ZERO`] will disable keep-alives."] # [doc = ""] # [doc = " By default, keep-alives are sent every 15 seconds."] # [must_use] pub fn with_keep_alive_interval (mut self , interval : Duration) -> Self { self . keep_alive_interval = interval ; self } }
};
}
