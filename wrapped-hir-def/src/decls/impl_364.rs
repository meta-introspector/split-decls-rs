macro_rules! deps {
    () => {
        DefMapCrateData!();
    };
}

macro_rules! impl_364 {
    () => {
        deps!();
        impl DefMapCrateData { fn new (edition : Edition) -> Self { Self { exported_derives : FxHashMap :: default () , fn_proc_macro_mapping : FxHashMap :: default () , registered_tools : PREDEFINED_TOOLS . iter () . map (| it | Symbol :: intern (it)) . collect () , unstable_features : FxHashSet :: default () , rustc_coherence_is_core : false , no_core : false , no_std : false , edition , recursion_limit : None , } } fn shrink_to_fit (& mut self) { let Self { exported_derives , fn_proc_macro_mapping , registered_tools , unstable_features , rustc_coherence_is_core : _ , no_core : _ , no_std : _ , edition : _ , recursion_limit : _ , } = self ; exported_derives . shrink_to_fit () ; fn_proc_macro_mapping . shrink_to_fit () ; registered_tools . shrink_to_fit () ; unstable_features . shrink_to_fit () ; } }
    };
}

impl_364!()