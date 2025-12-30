// Generated macro for hover (function)
macro_rules! Depcrate_hoverhover {
() => {
// Module: crate::hover
// Provides: {"hover"}
// Dependencies: {}
pub (crate) fn hover (db : & RootDatabase , frange @ FileRange { file_id , range } : FileRange , config : & HoverConfig < '_ > ,) -> Option < RangeInfo < HoverResult > > { let sema = & hir :: Semantics :: new (db) ; let file = sema . parse_guess_edition (file_id) . syntax () . clone () ; let edition = sema . attach_first_edition (file_id) . map (| it | it . edition (db)) . unwrap_or (Edition :: CURRENT) ; let display_target = sema . first_crate (file_id) ? . to_display_target (db) ; let mut res = if range . is_empty () { hover_offset (sema , FilePosition { file_id , offset : range . start () } , file , config , edition , display_target ,) } else { hover_ranged (sema , frange , file , config , edition , display_target) } ? ; if let HoverDocFormat :: PlainText = config . format { res . info . markup = remove_markdown (res . info . markup . as_str ()) . into () ; } Some (res) }
};
}
