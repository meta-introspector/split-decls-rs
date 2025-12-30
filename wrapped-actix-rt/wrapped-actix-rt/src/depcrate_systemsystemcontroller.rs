// Generated macro for SystemController (struct)
macro_rules! Depcrate_systemSystemController {
() => {
// Module: crate::system
// Provides: {"SystemController"}
// Dependencies: {}
# [doc = " There is one `SystemController` per [System]. It runs in the background, keeping track of"] # [doc = " [Arbiter]s and is able to distribute a system-wide stop command."] # [derive (Debug)] pub (crate) struct SystemController { stop_tx : Option < oneshot :: Sender < i32 > > , cmd_rx : mpsc :: UnboundedReceiver < SystemCommand > , arbiters : HashMap < usize , ArbiterHandle > , }
};
}
