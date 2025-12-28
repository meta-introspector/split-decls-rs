macro_rules! deps {
    () => {
        BagOfWordsVisitor!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'ast > Visit < 'ast > for BagOfWordsVisitor { fn visit_item_const (& mut self , i : & 'ast ItemConst) { self . add_ident_to_bag (& i . ident) ; syn :: visit :: visit_item_const (self , i) ; } fn visit_item_fn (& mut self , i : & 'ast ItemFn) { self . add_ident_to_bag (& i . sig . ident) ; syn :: visit :: visit_item_fn (self , i) ; } fn visit_item_struct (& mut self , i : & 'ast ItemStruct) { self . add_ident_to_bag (& i . ident) ; for field in & i . fields { if let Some (ident) = & field . ident { self . add_ident_to_bag (ident) ; } } syn :: visit :: visit_item_struct (self , i) ; } fn visit_item_enum (& mut self , i : & 'ast ItemEnum) { self . add_ident_to_bag (& i . ident) ; for variant in & i . variants { self . add_ident_to_bag (& variant . ident) ; for field in & variant . fields { if let Some (ident) = & field . ident { self . add_ident_to_bag (ident) ; } } } syn :: visit :: visit_item_enum (self , i) ; } fn visit_item_static (& mut self , i : & 'ast ItemStatic) { self . add_ident_to_bag (& i . ident) ; syn :: visit :: visit_item_static (self , i) ; } }
    };
}

impl_9!();