// Generated macro for Counts (struct)
macro_rules! Depcrate_proto_streams_countsCounts {
() => {
// Module: crate::proto::streams::counts
// Provides: {"Counts"}
// Dependencies: {}
# [derive (Debug)] pub (super) struct Counts { # [doc = " Acting as a client or server. This allows us to track which values to"] # [doc = " inc / dec."] peer : peer :: Dyn , # [doc = " Maximum number of locally initiated streams"] max_send_streams : usize , # [doc = " Current number of remote initiated streams"] num_send_streams : usize , # [doc = " Maximum number of remote initiated streams"] max_recv_streams : usize , # [doc = " Current number of locally initiated streams"] num_recv_streams : usize , # [doc = " Maximum number of pending locally reset streams"] max_local_reset_streams : usize , # [doc = " Current number of pending locally reset streams"] num_local_reset_streams : usize , # [doc = " Max number of \"pending accept\" streams that were remotely reset"] max_remote_reset_streams : usize , # [doc = " Current number of \"pending accept\" streams that were remotely reset"] num_remote_reset_streams : usize , # [doc = " Maximum number of locally reset streams due to protocol error across"] # [doc = " the lifetime of the connection."] # [doc = ""] # [doc = " When this gets exceeded, we issue GOAWAYs."] max_local_error_reset_streams : Option < usize > , # [doc = " Total number of locally reset streams due to protocol error across the"] # [doc = " lifetime of the connection."] num_local_error_reset_streams : usize , }
};
}
