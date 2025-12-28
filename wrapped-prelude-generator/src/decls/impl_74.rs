macro_rules! deps {
    () => {
        DependencyCollector!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < 'ast > Visit < 'ast > for DependencyCollector { fn visit_path (& mut self , i : & 'ast Path) { for segment in & i . segments { self . add_dependency (& segment . ident) ; } visit :: visit_path (self , i) ; } fn visit_macro (& mut self , i : & 'ast Macro) { for segment in & i . path . segments { self . add_dependency (& segment . ident) ; } visit :: visit_macro (self , i) ; } fn visit_ident (& mut self , i : & 'ast Ident) { self . add_dependency (i) ; } fn visit_item (& mut self , i : & 'ast Item) { visit :: visit_item (self , i) ; } }
    };
}

impl_74!()