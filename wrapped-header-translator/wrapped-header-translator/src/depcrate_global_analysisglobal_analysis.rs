// Generated macro for global_analysis (function)
macro_rules! Depcrate_global_analysisglobal_analysis {
() => {
// Module: crate::global_analysis
// Provides: {"global_analysis"}
// Dependencies: {}
pub fn global_analysis (library : & mut Library , config : & Config) { let _span = info_span ! ("analyzing") . entered () ; let mut implementable_mapping = create_implementable_mapping (& library . module) ; for (external_name , external_data) in & library . data . external { implementable_mapping . insert (ItemTree :: from_id (external_data . clone () . into_id (external_name . clone ()) ,)) ; } let mut expected_bridged_types = config . libraries . values () . flat_map (| data | & data . class_data) . filter_map (| (name , data) | data . bridged_to . as_ref () . map (| bridged | (& * * name , bridged))) . filter (| (_ , bridged) | bridged . library_name () == library . link_name) . collect () ; let ident_mapping = create_ident_mapping (& library . module) ; update_module (& mut library . module , & implementable_mapping , & ident_mapping , & mut expected_bridged_types ,) ; if ! expected_bridged_types . is_empty () { warn ! ("too many bridged-to in config: {expected_bridged_types:?}") ; } }
};
}
