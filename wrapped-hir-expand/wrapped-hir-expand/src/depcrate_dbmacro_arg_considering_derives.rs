// Generated macro for macro_arg_considering_derives (function)
macro_rules! Depcrate_dbmacro_arg_considering_derives {
() => {
// Module: crate::db
// Provides: {"macro_arg_considering_derives"}
// Dependencies: {}
# [doc = " This resolves the [MacroCallId] to check if it is a derive macro if so get the [macro_arg] for the derive."] # [doc = " Other wise return the [macro_arg] for the macro_call_id."] # [doc = ""] # [doc = " This is not connected to the database so it does not cached the result. However, the inner [macro_arg] query is"] # [allow (deprecated)] fn macro_arg_considering_derives (db : & dyn ExpandDatabase , id : MacroCallId , kind : & MacroCallKind ,) -> MacroArgResult { match kind { MacroCallKind :: Derive { derive_macro_id , .. } => db . macro_arg (* derive_macro_id) , _ => db . macro_arg (id) , } }
};
}
