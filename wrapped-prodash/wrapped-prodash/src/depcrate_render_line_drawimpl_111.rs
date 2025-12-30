// Generated macro for impl_111 (impl)
macro_rules! Depcrate_render_line_drawimpl_111 {
() => {
// Module: crate::render::line::draw
// Provides: {"impl_111"}
// Dependencies: {}
impl State { pub (crate) fn update_from_progress (& mut self , progress : & impl Root) -> bool { progress . sorted_snapshot (& mut self . tree) ; let mut hasher = DefaultHasher :: new () ; self . tree . hash (& mut hasher) ; let cur_hash = hasher . finish () ; self . for_next_copy = progress . copy_new_messages (& mut self . messages , self . for_next_copy . take ()) . into () ; let changed = self . tree_hash != cur_hash ; self . tree_hash = cur_hash ; changed } pub (crate) fn clear (& mut self) { self . tree . clear () ; self . messages . clear () ; self . for_next_copy . take () ; } }
};
}
