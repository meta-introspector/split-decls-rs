// Generated macro for add_local_place_comments (function)
macro_rules! Depcrate_abi_commentsadd_local_place_comments {
() => {
// Module: crate::abi::comments
// Provides: {"add_local_place_comments"}
// Dependencies: {}
pub (super) fn add_local_place_comments < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , place : CPlace < 'tcx > , local : Local ,) { if ! fx . clif_comments . enabled () { return ; } let TyAndLayout { ty , layout } = place . layout () ; let rustc_abi :: LayoutData { size , align , .. } = layout . 0 . 0 ; let (kind , extra) = place . debug_comment () ; fx . add_global_comment (format ! ("{:<5} {:5} {:30} {:4}b {}{}{}" , kind , format ! ("{:?}" , local) , format ! ("{:?}" , ty) , size . bytes () , align . abi . bytes () , if extra . is_empty () { "" } else { "                " } , extra ,)) ; }
};
}
