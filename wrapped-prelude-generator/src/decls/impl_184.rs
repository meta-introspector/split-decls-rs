macro_rules! deps {
    () => {
        TypeCollector!();
        TypeInfo!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < 'a > Visit < 'a > for TypeCollector < 'a > { fn visit_type (& mut self , i : & 'a Type) { if let Type :: Path (type_path) = i { for segment in type_path . path . segments . iter () { let type_name = segment . ident . to_string () ; let entry = self . type_map . entry (type_name . clone ()) . or_insert_with (| | TypeInfo { count : 0 , layer : Some (0) , }) ; entry . count += 1 ; if is_complex_type (& type_name) { entry . layer = Some (1) ; } if let PathArguments :: AngleBracketed (angle_args) = & segment . arguments { for arg in angle_args . args . iter () { if let GenericArgument :: Type (inner_ty) = arg { self . visit_type (inner_ty) ; } } } } } syn :: visit :: visit_type (self , i) ; } fn visit_item_struct (& mut self , i : & 'a ItemStruct) { let struct_name = i . ident . to_string () ; let entry = self . type_map . entry (struct_name . clone ()) . or_insert_with (| | TypeInfo { count : 0 , layer : Some (0) , }) ; entry . count += 1 ; if contains_complex_attributes (i) || contains_complex_fields (i) { entry . layer = Some (1) ; } syn :: visit :: visit_item_struct (self , i) ; } fn visit_item_const (& mut self , i : & 'a ItemConst) { let const_name = i . ident . to_string () ; let entry = self . type_map . entry (const_name . clone ()) . or_insert_with (| | TypeInfo { count : 0 , layer : Some (0) , }) ; entry . count += 1 ; if contains_complex_attributes_for_const (i) || is_complex_type (& const_name) { entry . layer = Some (1) ; } syn :: visit :: visit_item_const (self , i) ; } }
    };
}

impl_184!()