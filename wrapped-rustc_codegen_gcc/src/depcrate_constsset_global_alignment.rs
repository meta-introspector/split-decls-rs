// Generated macro for set_global_alignment (function)
macro_rules! Depcrate_constsset_global_alignment {
() => {
// Module: crate::consts
// Provides: {"set_global_alignment"}
// Dependencies: {}
fn set_global_alignment < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , gv : LValue < 'gcc > , mut align : Align ,) { if let Some (min_global) = cx . sess () . target . min_global_align { align = Ord :: max (align , min_global) ; } gv . set_alignment (align . bytes () as i32) ; }
};
}
