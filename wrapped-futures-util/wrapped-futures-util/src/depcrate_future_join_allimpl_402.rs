// Generated macro for impl_402 (impl)
macro_rules! Depcrate_future_join_allimpl_402 {
() => {
// Module: crate::future::join_all
// Provides: {"impl_402"}
// Dependencies: {}
impl < F > Future for JoinAll < F > where F : Future , { type Output = Vec < F :: Output > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match & mut self . kind { JoinAllKind :: Small { elems } => { let mut all_done = true ; for elem in iter_pin_mut (elems . as_mut ()) { if elem . poll (cx) . is_pending () { all_done = false ; } } if all_done { let mut elems = mem :: replace (elems , Box :: pin ([])) ; let result = iter_pin_mut (elems . as_mut ()) . map (| e | e . take_output () . unwrap ()) . collect () ; Poll :: Ready (result) } else { Poll :: Pending } } # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] JoinAllKind :: Big { fut } => Pin :: new (fut) . poll (cx) , } } }
};
}
