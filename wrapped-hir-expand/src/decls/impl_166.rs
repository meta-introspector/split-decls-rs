macro_rules! deps {
    () => {
        ProcMacroLoadResult!();
        CrateProcMacros!();
        ProcMacros!();
        ProcMacrosBuilder!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl ProcMacrosBuilder { pub fn insert (& mut self , proc_macros_crate : CrateBuilderId , mut proc_macro : ProcMacroLoadResult ,) { if let Ok (proc_macros) = & mut proc_macro { proc_macros . sort_unstable_by (| proc_macro , proc_macro2 | { (proc_macro . name . as_str () , proc_macro . kind) . cmp (& (proc_macro2 . name . as_str () , proc_macro2 . kind)) }) ; } self . 0 . insert (proc_macros_crate , match proc_macro { Ok (it) => Arc :: new (CrateProcMacros (Ok (it . into_boxed_slice ()))) , Err (e) => Arc :: new (CrateProcMacros (Err (e))) , } ,) ; } pub (crate) fn build (self , crates_id_map : & CratesIdMap) -> ProcMacros { let mut map = self . 0 . into_iter () . map (| (krate , proc_macro) | (crates_id_map [& krate] , proc_macro)) . collect :: < FxHashMap < _ , _ > > () ; map . shrink_to_fit () ; ProcMacros (map) } }
    };
}

impl_166!();