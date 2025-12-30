// Generated macro for StreamTag (enum)
macro_rules! Depcrate_hazardous_aead_streamingStreamTag {
() => {
// Module: crate::hazardous::aead::streaming
// Provides: {"StreamTag"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [doc = " Tag that indicates the type of message."] pub enum StreamTag { # [doc = " A message with no special meaning."] Message , # [doc = " Marks that the message is the end of a set of messages. Allows the decrypting site to"] # [doc = " start working with this data."] Push , # [doc = " Derives a new secret key and forgets the one used for earlier encryption/decryption"] # [doc = " operations."] Rekey , # [doc = " Indicates the end of a stream. Also does a rekey."] Finish , }
};
}
