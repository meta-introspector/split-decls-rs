// Generated macro for set_global_alignment (function)
macro_rules! Depcrate_constsset_global_alignment {
() => {
// Module: crate::consts
// Provides: {"set_global_alignment"}
// Dependencies: {}
fn set_global_alignment < 'll > (cx : & CodegenCx < 'll , '_ > , gv : & 'll Value , mut align : Align) { if let Some (min_global) = cx . sess () . target . min_global_align { align = Ord :: max (align , min_global) ; } llvm :: set_alignment (gv , align) ; }
};
}
