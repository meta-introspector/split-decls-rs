// Generated macro for Capabilities (struct)
macro_rules! Depcrate_client_capabilitiesCapabilities {
() => {
// Module: crate::client::capabilities
// Provides: {"Capabilities"}
// Dependencies: {}
# [doc = " A structure to represent multiple [capabilities](Capability) or features supported by the server."] # [doc = ""] # [doc = " ### Deviation"] # [doc = ""] # [doc = " As a *shortcoming*, we are unable to parse `V1` as emitted from `git-upload-pack` without a `git-daemon` or server,"] # [doc = " as it will not emit any capabilities for some reason. Only `V2` and `V0` work in that context."] # [derive (Debug , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Capabilities { data : BString , value_sep : u8 , }
};
}
