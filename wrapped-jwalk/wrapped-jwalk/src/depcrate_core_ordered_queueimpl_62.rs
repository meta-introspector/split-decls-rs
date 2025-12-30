// Generated macro for impl_62 (impl)
macro_rules! Depcrate_core_ordered_queueimpl_62 {
() => {
// Module: crate::core::ordered_queue
// Provides: {"impl_62"}
// Dependencies: {}
impl < T > OrderedQueueIter < T > where T : Send , { fn pending_count (& self) -> usize { self . pending_count . load (AtomicOrdering :: SeqCst) } fn is_stop (& self) -> bool { self . stop . load (AtomicOrdering :: SeqCst) } fn try_next_relaxed (& mut self) -> Result < Ordered < T > , TryRecvError > { if self . is_stop () { return Err (TryRecvError :: Disconnected) ; } while let Ok (ordered_work) = self . receiver . try_recv () { self . receive_buffer . push (ordered_work) } if let Some (ordered_work) = self . receive_buffer . pop () { Ok (ordered_work) } else if self . pending_count () == 0 { Err (TryRecvError :: Disconnected) } else { Err (TryRecvError :: Empty) } } fn try_next_strict (& mut self) -> Result < Ordered < T > , TryRecvError > { let looking_for = & self . ordered_matcher . looking_for ; loop { if self . is_stop () { return Err (TryRecvError :: Disconnected) ; } let top_ordered = self . receive_buffer . peek () ; if let Some (top_ordered) = top_ordered { if top_ordered . index_path . eq (looking_for) { break ; } } if self . ordered_matcher . is_none () { return Err (TryRecvError :: Disconnected) ; } match self . receiver . try_recv () { Ok (ordered) => { self . receive_buffer . push (ordered) ; } Err (err) => match err { TryRecvError :: Empty => thread :: yield_now () , TryRecvError :: Disconnected => break , } , } } let ordered = self . receive_buffer . pop () . unwrap () ; self . ordered_matcher . advance_past (& ordered) ; Ok (ordered) } }
};
}
