// Generated macro for mode_to_flags (function)
macro_rules! Depcrate_os_kqueuemode_to_flags {
() => {
// Module: crate::os::kqueue
// Provides: {"mode_to_flags"}
// Dependencies: {}
pub (crate) fn mode_to_flags (mode : PollMode) -> kqueue :: EventFlags { use kqueue :: EventFlags as EV ; match mode { PollMode :: Oneshot => EV :: ONESHOT , PollMode :: Level => EV :: empty () , PollMode :: Edge => EV :: CLEAR , PollMode :: EdgeOneshot => EV :: ONESHOT | EV :: CLEAR , } }
};
}
