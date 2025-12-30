// Generated macro for adjust_simple_prj (function)
macro_rules! Depcrate_use_trackingadjust_simple_prj {
() => {
// Module: crate::use_tracking
// Provides: {"adjust_simple_prj"}
// Dependencies: {}
fn adjust_simple_prj (tpath : & syn :: TypePath) -> syn :: TypePath { let segments = tpath . qself . as_ref () . filter (| qp | qp . as_token . is_none ()) . and_then (| qp | extract_path (& * qp . ty)) . filter (| tp | tp . qself . is_none ()) . map (| tp | & tp . path . segments) ; if let Some (segments) = segments { let tpath = tpath . clone () ; let mut segments = segments . clone () ; segments . push_punct (< Token ! [::] > :: default ()) ; segments . extend (tpath . path . segments . into_pairs ()) ; syn :: TypePath { qself : None , path : syn :: Path { leading_colon : None , segments , } , } } else { tpath . clone () } }
};
}
