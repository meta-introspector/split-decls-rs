macro_rules! is_doc_alias_attrs_contain_symbol {
    () => {
        pub fn is_doc_alias_attrs_contain_symbol < 'tcx , T : AttributeExt + 'tcx > (attrs : impl Iterator < Item = & 'tcx T > , symbol : Symbol ,) -> bool { let doc_attrs = attrs . filter (| attr | attr . has_name (sym :: doc)) ; for attr in doc_attrs { let Some (values) = attr . meta_item_list () else { continue ; } ; let alias_values = values . iter () . filter (| v | v . has_name (sym :: alias)) ; for v in alias_values { if let Some (nested) = v . meta_item_list () { let mut iter = nested . iter () . filter_map (| item | item . lit ()) . map (| item | item . symbol) ; if iter . any (| s | s == symbol) { return true ; } } else if let Some (meta) = v . meta_item () && let Some (lit) = meta . name_value_literal () { if lit . symbol == symbol { return true ; } } } } false }
    };
}

is_doc_alias_attrs_contain_symbol!()