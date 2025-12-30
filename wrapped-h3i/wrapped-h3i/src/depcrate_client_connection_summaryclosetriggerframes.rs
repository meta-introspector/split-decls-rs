// Generated macro for CloseTriggerFrames (struct)
macro_rules! Depcrate_client_connection_summaryCloseTriggerFrames {
() => {
// Module: crate::client::connection_summary
// Provides: {"CloseTriggerFrames"}
// Dependencies: {}
# [doc = " A container for frames that h3i expects to see over a given connection. If"] # [doc = " h3i receives all the frames it expects, it will send a CONNECTION_CLOSE"] # [doc = " frame to the server. This bypasses the idle timeout and vastly quickens test"] # [doc = " suites which depend heavily on h3i."] # [doc = ""] # [doc = " The specific CONNECTION_CLOSE frame can be customized by passing a"] # [doc = " [`ConnectionError`] to [`Self::new_with_connection_close`]. h3i will send an"] # [doc = " application CONNECTION_CLOSE frame with error code 0x100 if this struct is"] # [doc = " constructed with the [`Self::new`] constructor."] # [derive (Clone , Serialize , Debug)] pub struct CloseTriggerFrames { missing : Vec < CloseTriggerFrame > , # [serde (skip)] close_with : ConnectionError , }
};
}
