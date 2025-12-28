macro_rules! deps {
    () => {
        BuiltinConstNoMangle!();
        LateContext!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for InvalidNoMangleItems { fn check_item (& mut self , cx : & LateContext < '_ > , it : & hir :: Item < '_ >) { let attrs = cx . tcx . hir_attrs (it . hir_id ()) ; match it . kind { hir :: ItemKind :: Fn { .. } => { if let Some (attr_span) = find_attr ! (attrs , AttributeKind :: ExportName { span , .. } => * span) . or_else (| | find_attr ! (attrs , AttributeKind :: NoMangle (span) => * span)) { self . check_no_mangle_on_generic_fn (cx , attr_span , it . owner_id . def_id) ; } } hir :: ItemKind :: Const (..) => { if find_attr ! (attrs , AttributeKind :: NoMangle (..)) { let start = cx . tcx . sess . source_map () . span_to_snippet (it . span) . map (| snippet | snippet . find ("const") . unwrap_or (0)) . unwrap_or (0) as u32 ; let suggestion = it . span . with_hi (BytePos (it . span . lo () . 0 + start + 5)) ; cx . emit_span_lint (NO_MANGLE_CONST_ITEMS , it . span , BuiltinConstNoMangle { suggestion } ,) ; } } _ => { } } } fn check_impl_item (& mut self , cx : & LateContext < '_ > , it : & hir :: ImplItem < '_ >) { let attrs = cx . tcx . hir_attrs (it . hir_id ()) ; match it . kind { hir :: ImplItemKind :: Fn { .. } => { if let Some (attr_span) = find_attr ! (attrs , AttributeKind :: ExportName { span , .. } => * span) . or_else (| | find_attr ! (attrs , AttributeKind :: NoMangle (span) => * span)) { self . check_no_mangle_on_generic_fn (cx , attr_span , it . owner_id . def_id) ; } } _ => { } } } }
    };
}

impl_50!()