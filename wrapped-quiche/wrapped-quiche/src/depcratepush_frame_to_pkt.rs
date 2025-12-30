// Generated macro for push_frame_to_pkt (macro)
macro_rules! Depcratepush_frame_to_pkt {
() => {
// Module: crate
// Provides: {"push_frame_to_pkt"}
// Dependencies: {}
# [doc = " Pushes a frame to the output packet if there is enough space."] # [doc = ""] # [doc = " Returns `true` on success, `false` otherwise. In case of failure it means"] # [doc = " there is no room to add the frame in the packet. You may retry to add the"] # [doc = " frame later."] macro_rules ! push_frame_to_pkt { ($ out : expr , $ frames : expr , $ frame : expr , $ left : expr) => { { if $ frame . wire_len () <= $ left { $ left -= $ frame . wire_len () ; $ frame . to_bytes (& mut $ out) ?; $ frames . push ($ frame) ; true } else { false } } } ; }
};
}
