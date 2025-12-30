// Generated macro for connect (function)
macro_rules! Depcrate_client_blocking_io_fileconnect {
() => {
// Module: crate::client::blocking_io::file
// Provides: {"connect"}
// Dependencies: {}
# [doc = " Connect to a locally readable repository at `path` using the given `desired_version`."] # [doc = " If `trace` is `true`, all packetlines received or sent will be passed to the facilities of the `gix-trace` crate."] # [doc = ""] # [doc = " This will spawn a `git` process locally."] pub fn connect (path : impl Into < BString > , desired_version : Protocol , trace : bool ,) -> Result < SpawnProcessOnDemand , std :: convert :: Infallible > { Ok (SpawnProcessOnDemand :: new_local (path . into () , desired_version , trace)) }
};
}
