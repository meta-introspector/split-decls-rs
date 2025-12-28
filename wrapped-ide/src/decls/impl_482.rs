macro_rules! deps {
    () => {
        Edge!();
        DotCrateGraph!();
    };
}

macro_rules! impl_482 {
    () => {
        deps!();
        impl < 'a > dot :: GraphWalk < 'a , Crate , Edge < 'a > > for DotCrateGraph < '_ > { fn nodes (& 'a self) -> dot :: Nodes < 'a , Crate > { self . crates_to_render . keys () . copied () . collect () } fn edges (& 'a self) -> dot :: Edges < 'a , Edge < 'a > > { self . crates_to_render . iter () . flat_map (| (krate , (crate_data , _)) | { crate_data . dependencies . iter () . filter (| dep | self . crates_to_render . contains_key (& dep . crate_id)) . map (move | dep | (* krate , dep)) }) . collect () } fn source (& 'a self , edge : & Edge < 'a >) -> Crate { edge . 0 } fn target (& 'a self , edge : & Edge < 'a >) -> Crate { edge . 1 . crate_id } }
    };
}

impl_482!()