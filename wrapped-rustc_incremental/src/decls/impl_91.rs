macro_rules! deps {
    () => {
        FindAllAttrs!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < 'tcx > intravisit :: Visitor < 'tcx > for FindAllAttrs < 'tcx > { type NestedFilter = nested_filter :: All ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_attribute (& mut self , attr : & 'tcx Attribute) { if self . is_active_attr (attr) { self . found_attrs . push (attr) ; } } }
    };
}

impl_91!()