macro_rules! deps {
    () => {
        StructLatticeInfo!();
        ImplLatticeInfo!();
        ImplMethodCoOccurrenceVisitor!();
        EnumVariantCoOccurrenceVisitor!();
        StructFieldCoOccurrenceVisitor!();
        TypeUsageVisitor!();
        EnumLatticeInfo!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < 'ast > Visit < 'ast > for TypeUsageVisitor { fn visit_expr (& mut self , i : & 'ast Expr) { self . current_depth += 1 ; self . process_expression (i) ; visit :: visit_expr (self , i) ; self . current_depth -= 1 ; } fn visit_item_fn (& mut self , i : & 'ast ItemFn) { self . current_depth += 1 ; visit :: visit_item_fn (self , i) ; self . current_depth -= 1 ; } fn visit_item_struct (& mut self , i : & 'ast ItemStruct) { self . current_depth += 1 ; let struct_name = i . ident . to_string () ; self . struct_lattices . entry (struct_name . clone ()) . or_insert_with (| | StructLatticeInfo :: new (struct_name)) ; visit :: visit_item_struct (self , i) ; self . current_depth -= 1 ; } fn visit_item_enum (& mut self , i : & 'ast ItemEnum) { self . current_depth += 1 ; let enum_name = i . ident . to_string () ; let enum_lattice_info = self . enum_lattices . entry (enum_name . clone ()) . or_insert_with (| | EnumLatticeInfo :: new (enum_name . clone ())) ; let mut sub_visitor = EnumVariantCoOccurrenceVisitor { _enum_name : & enum_name , _current_variant_types : BTreeSet :: new () , enum_lattice_info , } ; sub_visitor . visit_item_enum (i) ; visit :: visit_item_enum (self , i) ; self . current_depth -= 1 ; } fn visit_item_const (& mut self , i : & 'ast ItemConst) { self . current_depth += 1 ; visit :: visit_item_const (self , i) ; self . current_depth -= 1 ; } fn visit_item_static (& mut self , i : & 'ast ItemStatic) { self . current_depth += 1 ; visit :: visit_item_static (self , i) ; self . current_depth -= 1 ; } fn visit_item_impl (& mut self , i : & 'ast ItemImpl) { self . current_depth += 1 ; let impl_for_type = if let Type :: Path (type_path) = & * i . self_ty { type_path . path . segments . last () . map (| segment | segment . ident . to_string ()) } else { None } ; if let Some (impl_for_type_name) = impl_for_type { let impl_lattice_info = self . impl_lattices . entry (impl_for_type_name . clone ()) . or_insert_with (| | ImplLatticeInfo :: new (impl_for_type_name . clone ())) ; let mut sub_visitor = ImplMethodCoOccurrenceVisitor { _impl_for_type : & impl_for_type_name , current_method_calls : BTreeSet :: new () , impl_lattice_info , } ; sub_visitor . visit_item_impl (i) ; if let Some (struct_lattice_info) = self . struct_lattices . get_mut (& impl_for_type_name) { let mut sub_visitor = StructFieldCoOccurrenceVisitor { _struct_name : & impl_for_type_name , current_field_accesses : BTreeSet :: new () , struct_lattice_info , } ; sub_visitor . visit_item_impl (i) ; } } visit :: visit_item_impl (self , i) ; self . current_depth -= 1 ; } }
    };
}

impl_208!();