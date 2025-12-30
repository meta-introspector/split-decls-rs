// Generated macro for Kind (enum)
macro_rules! Depcrate_errorKind {
() => {
// Module: crate::error
// Provides: {"Kind"}
// Dependencies: {}
# [derive (Debug)] enum Kind { # [doc = " A RST_STREAM frame was received or sent."] # [allow (dead_code)] Reset (StreamId , Reason , Initiator) , # [doc = " A GO_AWAY frame was received or sent."] GoAway (Bytes , Reason , Initiator) , # [doc = " The user created an error from a bare Reason."] Reason (Reason) , # [doc = " An error resulting from an invalid action taken by the user of this"] # [doc = " library."] User (UserError) , # [doc = " An `io::Error` occurred while trying to read or write."] Io (io :: Error) , }
};
}
