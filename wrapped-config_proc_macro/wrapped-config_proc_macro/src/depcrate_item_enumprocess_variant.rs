// Generated macro for process_variant (function)
macro_rules! Depcrate_item_enumprocess_variant {
() => {
// Module: crate::item_enum
// Provides: {"process_variant"}
// Dependencies: {}
# [doc = " Remove attributes specific to `config_proc_macro` from enum variant fields."] fn process_variant (variant : & syn :: Variant) -> TokenStream { let metas = variant . attrs . iter () . filter (| attr | ! is_doc_hint (attr) && ! is_config_value (attr) && ! is_unstable_variant (attr)) ; let attrs = fold_quote (metas , | meta | quote ! (# meta)) ; let syn :: Variant { ident , fields , .. } = variant ; quote ! (# attrs # ident # fields) }
};
}
