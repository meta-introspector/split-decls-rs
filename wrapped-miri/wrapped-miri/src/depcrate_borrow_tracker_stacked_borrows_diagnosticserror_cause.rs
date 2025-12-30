// Generated macro for error_cause (function)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_diagnosticserror_cause {
() => {
// Module: crate::borrow_tracker::stacked_borrows::diagnostics
// Provides: {"error_cause"}
// Dependencies: {}
fn error_cause (stack : & Stack , prov_extra : ProvenanceExtra) -> & 'static str { if let ProvenanceExtra :: Concrete (tag) = prov_extra { if (0 .. stack . len ()) . map (| i | stack . get (i) . unwrap ()) . any (| item | item . tag () == tag && item . perm () != Permission :: Disabled) { ", but that tag only grants SharedReadOnly permission for this location" } else { ", but that tag does not exist in the borrow stack for this location" } } else { ", but no exposed tags have suitable permission in the borrow stack for this location" } }
};
}
