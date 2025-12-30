// Generated macro for ValidityVisitor (struct)
macro_rules! Depcrate_interpret_validityValidityVisitor {
() => {
// Module: crate::interpret::validity
// Provides: {"ValidityVisitor"}
// Dependencies: {}
struct ValidityVisitor < 'rt , 'tcx , M : Machine < 'tcx > > { # [doc = " The `path` may be pushed to, but the part that is present when a function"] # [doc = " starts must not be changed!  `visit_fields` and `visit_array` rely on"] # [doc = " this stack discipline."] path : Vec < PathElem > , ref_tracking : Option < & 'rt mut RefTracking < MPlaceTy < 'tcx , M :: Provenance > , Vec < PathElem > > > , # [doc = " `None` indicates this is not validating for CTFE (but for runtime)."] ctfe_mode : Option < CtfeValidationMode > , ecx : & 'rt mut InterpCx < 'tcx , M > , # [doc = " Whether provenance should be reset outside of pointers (emulating the effect of a typed"] # [doc = " copy)."] reset_provenance_and_padding : bool , # [doc = " This tracks which byte ranges in this value contain data; the remaining bytes are padding."] # [doc = " The ideal representation here would be pointer-length pairs, but to keep things more compact"] # [doc = " we only store a (range) set of offsets -- the base pointer is the same throughout the entire"] # [doc = " visit, after all."] # [doc = " If this is `Some`, then `reset_provenance_and_padding` must be true (but not vice versa:"] # [doc = " we might not track data vs padding bytes if the operand isn't stored in memory anyway)."] data_bytes : Option < RangeSet > , }
};
}
