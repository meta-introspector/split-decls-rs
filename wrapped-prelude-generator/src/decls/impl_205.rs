macro_rules! deps {
    () => {
        ImplMethodCoOccurrenceVisitor!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < 'a , 'ast > Visit < 'ast > for ImplMethodCoOccurrenceVisitor < 'a > { fn visit_expr_method_call (& mut self , i : & 'ast syn :: ExprMethodCall) { self . current_method_calls . insert (i . method . to_string ()) ; visit :: visit_expr_method_call (self , i) ; } fn visit_item_fn (& mut self , i : & 'ast ItemFn) { self . current_method_calls . clear () ; visit :: visit_item_fn (self , i) ; if ! self . current_method_calls . is_empty () { self . impl_lattice_info . add_co_occurrence (self . current_method_calls . clone ()) ; } } }
    };
}

impl_205!();