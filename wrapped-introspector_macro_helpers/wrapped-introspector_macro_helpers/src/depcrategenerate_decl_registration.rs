// Generated macro for generate_decl_registration (macro)
macro_rules! Depcrategenerate_decl_registration {
() => {
// Module: crate
// Provides: {"generate_decl_registration"}
// Dependencies: {}
# [macro_export] macro_rules ! generate_decl_registration { ($ node_type : expr , $ name_expr : expr , $ vis_expr : expr , $ module_path_expr : expr , $ file_expr : expr , $ line_expr : expr , $ hash_expr : expr) => { { use introspector_decl_common :: { DeclInfo , register_decl } ; register_decl (DeclInfo { node_type : $ node_type , name : $ name_expr , visibility : $ vis_expr , module : $ module_path_expr , file : $ file_expr , line : $ line_expr , hash : $ hash_expr , }) ; } } ; }
};
}
