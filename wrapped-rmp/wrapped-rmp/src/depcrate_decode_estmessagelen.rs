// Generated macro for MessageLen (struct)
macro_rules! Depcrate_decode_estMessageLen {
() => {
// Module: crate::decode::est
// Provides: {"MessageLen"}
// Dependencies: {}
# [doc = " Incremental MessagePack parser that can parse incomplete messages,"] # [doc = " and report their estimated total length."] pub struct MessageLen { # [doc = " The last operation interrupted"] wip : Option < WIP > , # [doc = " Max size estimate"] max_position : NonZeroUsize , # [doc = " Bytes read so far"] position : usize , # [doc = " Stack of open arrays and maps"] # [doc = " It is not a complete stack. Used only when resumption is needed."] sequences_wip : Vec < Seq > , # [doc = " Nesting of arrays and maps"] current_depth : u16 , # [doc = " Configured limit"] max_depth : u16 , # [doc = " Configured limit"] max_len : u32 , }
};
}
