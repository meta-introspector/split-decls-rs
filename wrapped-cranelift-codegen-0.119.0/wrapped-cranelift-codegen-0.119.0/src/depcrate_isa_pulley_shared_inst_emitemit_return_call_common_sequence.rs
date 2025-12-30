// Generated macro for emit_return_call_common_sequence (function)
macro_rules! Depcrate_isa_pulley_shared_inst_emitemit_return_call_common_sequence {
() => {
// Module: crate::isa::pulley_shared::inst::emit
// Provides: {"emit_return_call_common_sequence"}
// Dependencies: {}
fn emit_return_call_common_sequence < T , P > (sink : & mut MachBuffer < InstAndKind < P > > , emit_info : & EmitInfo , state : & mut EmitState < P > , info : & ReturnCallInfo < T > ,) where P : PulleyTargetKind , { let mut buffer = MachBuffer :: new () ; let mut fake_emit_state = state . clone () ; return_call_emit_impl (& mut buffer , emit_info , & mut fake_emit_state , info) ; let buffer = buffer . finish (& Default :: default () , & mut Default :: default ()) ; let length = buffer . data () . len () as u32 ; if sink . island_needed (length) { let jump_around_label = sink . get_label () ; < InstAndKind < P > > :: gen_jump (jump_around_label) . emit (sink , emit_info , state) ; sink . emit_island (length + 4 , & mut state . ctrl_plane) ; sink . bind_label (jump_around_label , & mut state . ctrl_plane) ; } return_call_emit_impl (sink , emit_info , state , info) ; }
};
}
