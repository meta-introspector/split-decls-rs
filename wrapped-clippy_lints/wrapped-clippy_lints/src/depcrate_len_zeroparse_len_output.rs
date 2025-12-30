// Generated macro for parse_len_output (function)
macro_rules! Depcrate_len_zeroparse_len_output {
() => {
// Module: crate::len_zero
// Provides: {"parse_len_output"}
// Dependencies: {}
fn parse_len_output < 'tcx > (cx : & LateContext < 'tcx > , sig : FnSig < 'tcx >) -> Option < LenOutput > { if let Some (segment) = extract_future_output (cx , sig . output ()) { let res = segment . res ; if matches ! (res , Res :: PrimTy (PrimTy :: Uint (_) | PrimTy :: Int (_))) { return Some (LenOutput :: Integral) ; } if let Res :: Def (_ , def_id) = res && let Some (res) = match cx . tcx . get_diagnostic_name (def_id) { Some (sym :: Option) => Some (LenOutput :: Option (def_id)) , Some (sym :: Result) => Some (LenOutput :: Result (def_id)) , _ => None , } && is_first_generic_integral (segment) { return Some (res) ; } return None ; } match * sig . output () . kind () { ty :: Int (_) | ty :: Uint (_) => Some (LenOutput :: Integral) , ty :: Adt (adt , subs) => match cx . tcx . get_diagnostic_name (adt . did ()) { Some (sym :: Option) => subs . type_at (0) . is_integral () . then (| | LenOutput :: Option (adt . did ())) , Some (sym :: Result) => subs . type_at (0) . is_integral () . then (| | LenOutput :: Result (adt . did ())) , _ => None , } , _ => None , } }
};
}
