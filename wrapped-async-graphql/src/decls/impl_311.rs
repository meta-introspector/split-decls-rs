macro_rules! deps {
    () => {
        Field!();
        VisitorContext!();
        MetaType!();
        Visitor!();
        CacheControlCalculate!();
        Object!();
        VisitMode!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        impl Visitor < '_ > for CacheControlCalculate < '_ > { fn mode (& self) -> VisitMode { VisitMode :: Inline } fn enter_selection_set (& mut self , ctx : & mut VisitorContext < '_ > , _selection_set : & Positioned < SelectionSet > ,) { if let Some (MetaType :: Object { cache_control , .. }) = ctx . current_type () { * self . cache_control = self . cache_control . merge (cache_control) ; } } fn enter_field (& mut self , ctx : & mut VisitorContext < '_ > , field : & Positioned < Field >) { if let Some (registry_field) = ctx . parent_type () . and_then (| parent | parent . field_by_name (& field . node . name . node)) { * self . cache_control = self . cache_control . merge (& registry_field . cache_control) ; } } }
    };
}

impl_311!();