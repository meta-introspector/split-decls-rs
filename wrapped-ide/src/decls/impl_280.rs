macro_rules! deps {
    () => {
        InlayHintLabelBuilder!();
        LazyProperty!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl HirWrite for InlayHintLabelBuilder < '_ > { fn start_location_link (& mut self , def : ModuleDefId) { never ! (self . location . is_some () , "location link is already started") ; self . make_new_part () ; self . location = Some (if self . resolve { LazyProperty :: Lazy } else { LazyProperty :: Computed ({ let Some (location) = ModuleDef :: from (def) . try_to_nav (self . sema) else { return } ; let location = location . call_site () ; FileRange { file_id : location . file_id , range : location . focus_or_full_range () } }) }) ; } fn end_location_link (& mut self) { self . make_new_part () ; } }
    };
}

impl_280!()