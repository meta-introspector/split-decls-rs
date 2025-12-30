// Generated macro for level_prefix (function)
macro_rules! Depcrate_render_tui_draw_progresslevel_prefix {
() => {
// Module: crate::render::tui::draw::progress
// Provides: {"level_prefix"}
// Dependencies: {}
fn level_prefix (entries : & [(Key , Task)] , entry_index : usize) -> String { let adj = Key :: adjacency (entries , entry_index) ; let key = entries [entry_index] . 0 ; let key_level = key . level () ; let is_orphan = adj . level () != key_level ; let mut buf = String :: with_capacity (key_level as usize) ; for level in 1 ..= key_level { use crate :: progress :: key :: SiblingLocation :: * ; let is_child_level = level == key_level ; if level != 1 { buf . push (' ') ; } if level == 1 && is_child_level { buf . push (match adj [level] { AboveAndBelow | Above => '├' , NotFound | Below => '│' , }) ; } else { let c = if is_child_level { match adj [level] { NotFound => { if is_orphan { ' ' } else { '·' } } Above => '└' , Below => '┌' , AboveAndBelow => '├' , } } else { match adj [level] { NotFound => { if level == 1 { '│' } else if is_orphan { '·' } else { ' ' } } Above => '└' , Below => '┌' , AboveAndBelow => '│' , } } ; buf . push (c) } } buf }
};
}
