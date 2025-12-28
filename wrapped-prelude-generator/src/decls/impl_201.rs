macro_rules! deps {
    () => {
        StructFieldCoOccurrenceVisitor!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl < 'a , 'ast > Visit < 'ast > for StructFieldCoOccurrenceVisitor < 'a > { fn visit_expr_field (& mut self , i : & 'ast syn :: ExprField) { if let Some (field_ident) = i . member . clone () . into_token_stream () . to_string () . strip_prefix (".") . map (| s | s . to_string ()) { self . current_field_accesses . insert (field_ident) ; } visit :: visit_expr_field (self , i) ; } fn visit_item_fn (& mut self , i : & 'ast ItemFn) { self . current_field_accesses . clear () ; visit :: visit_item_fn (self , i) ; if ! self . current_field_accesses . is_empty () { self . struct_lattice_info . add_co_occurrence (self . current_field_accesses . clone ()) ; } } }
    };
}

impl_201!();