// Generated macro for State (struct)
macro_rules! Depcrate_render_line_drawState {
() => {
// Module: crate::render::line::draw
// Provides: {"State"}
// Dependencies: {}
# [derive (Default)] pub struct State { tree : Vec < (progress :: Key , progress :: Task) > , tree_hash : u64 , messages : Vec < Message > , for_next_copy : Option < MessageCopyState > , # [doc = " The size of the message origin, tracking the terminal height so things potentially off screen don't influence width anymore."] message_origin_size : VecDeque < usize > , # [doc = " The maximum progress midpoint (point till progress bar starts) seen at the last tick"] last_progress_midpoint : Option < u16 > , # [doc = " The amount of blocks per line we have written last time."] blocks_per_line : VecDeque < u16 > , pub throughput : Option < Throughput > , }
};
}
