// Generated macro for ConnectionConfig (struct)
macro_rules! DepcrateConnectionConfig {
() => {
// Module: crate
// Provides: {"ConnectionConfig"}
// Dependencies: {}
# [doc = " ConnectionConfig is used to configure the connection once the client sends the ConnectionInit"] # [doc = " message."] # [derive (Clone , Copy , Debug)] pub struct ConnectionConfig < CtxT > { # [doc = " Custom-provided [`juniper::Context`]."] pub context : CtxT , # [doc = " Maximum number of in-flight operations that a connection can have."] # [doc = ""] # [doc = " If this number is exceeded, attempting to start more will result in an error."] # [doc = " By default, there is no limit to in-flight operations."] pub max_in_flight_operations : usize , # [doc = " Interval at which to send keep-alives."] # [doc = ""] # [doc = " Specifying a [`Duration::ZERO`] will disable keep-alives."] # [doc = ""] # [doc = " By default, keep-alives are sent every 15 seconds."] pub keep_alive_interval : Duration , }
};
}
