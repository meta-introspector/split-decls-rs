macro_rules! deps {
    () => {
        Edge!();
        DotCrateGraph!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        impl < 'a > dot :: Labeller < 'a , Crate , Edge < 'a > > for DotCrateGraph < '_ > { fn graph_id (& 'a self) -> Id < 'a > { Id :: new ("rust_analyzer_crate_graph") . unwrap () } fn node_id (& 'a self , n : & Crate) -> Id < 'a > { let id = n . as_id () . index () ; Id :: new (format ! ("_{id:?}")) . unwrap () } fn node_shape (& 'a self , _node : & Crate) -> Option < LabelText < 'a > > { Some (LabelText :: LabelStr ("box" . into ())) } fn node_label (& 'a self , n : & Crate) -> LabelText < 'a > { let name = self . crates_to_render [n] . 1 . display_name . as_ref () . map_or ("(unnamed crate)" , | name | name . as_str ()) ; LabelText :: LabelStr (name . into ()) } }
    };
}

impl_483!()