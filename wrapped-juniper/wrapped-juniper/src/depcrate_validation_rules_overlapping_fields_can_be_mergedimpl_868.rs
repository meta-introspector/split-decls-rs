// Generated macro for impl_868 (impl)
macro_rules! Depcrate_validation_rules_overlapping_fields_can_be_mergedimpl_868 {
() => {
// Module: crate::validation::rules::overlapping_fields_can_be_merged
// Provides: {"impl_868"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for OverlappingFieldsCanBeMerged < 'a , S > where S : ScalarValue , { fn enter_document (& mut self , _ : & mut ValidatorContext < 'a , S > , defs : & 'a Document < S >) { for def in defs { if let Definition :: Fragment (Spanning { ref item , .. }) = * def { self . named_fragments . insert (item . name . item , item) ; } } } fn enter_selection_set (& mut self , ctx : & mut ValidatorContext < 'a , S > , selection_set : & 'a [Selection < S >] ,) { for Conflict (ConflictReason (reason_name , reason_msg) , mut p1 , mut p2) in self . find_conflicts_within_selection_set (ctx . parent_type () , selection_set , ctx) { p1 . append (& mut p2) ; ctx . report_error (& error_message (& reason_name , & reason_msg) , & p1) ; } } }
};
}
