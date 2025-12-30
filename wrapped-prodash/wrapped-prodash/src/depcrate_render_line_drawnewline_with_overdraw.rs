// Generated macro for newline_with_overdraw (function)
macro_rules! Depcrate_render_line_drawnewline_with_overdraw {
() => {
// Module: crate::render::line::draw
// Provides: {"newline_with_overdraw"}
// Dependencies: {}
# [doc = " Must be called directly after `tokens` were drawn, without newline. Takes care of adding the newline."] fn newline_with_overdraw (out : & mut impl io :: Write , tokens : & [ANSIString < '_ >] , blocks_in_last_iteration : u16 ,) -> io :: Result < u16 > { let current_block_count = block_count_sans_ansi_codes (tokens) ; if blocks_in_last_iteration > current_block_count { writeln ! (out , "{:>width$}" , "" , width = (blocks_in_last_iteration - current_block_count) as usize) ? ; } else { writeln ! (out) ? ; } ; Ok (current_block_count) }
};
}
