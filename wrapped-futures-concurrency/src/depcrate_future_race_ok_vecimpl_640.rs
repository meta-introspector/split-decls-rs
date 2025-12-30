// Generated macro for impl_640 (impl)
macro_rules! Depcrate_future_race_ok_vecimpl_640 {
() => {
// Module: crate::future::race_ok::vec
// Provides: {"impl_640"}
// Dependencies: {}
impl < Fut , T , E > Future for RaceOk < Fut , T , E > where Fut : Future < Output = Result < T , E > > , { type Output = Result < T , AggregateError < E > > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut all_done = true ; for mut elem in iter_pin_mut (self . elems . as_mut ()) { if elem . as_mut () . poll (cx) . is_pending () { all_done = false } else if let Some (output) = elem . take_ok () { return Poll :: Ready (Ok (output)) ; } } if all_done { let mut elems = mem :: replace (& mut self . elems , Box :: pin ([])) ; let result : Vec < E > = iter_pin_mut (elems . as_mut ()) . map (| e | match e . take_err () { Some (err) => err , None => unreachable ! () , }) . collect () ; Poll :: Ready (Err (AggregateError :: new (result))) } else { Poll :: Pending } } }
};
}
