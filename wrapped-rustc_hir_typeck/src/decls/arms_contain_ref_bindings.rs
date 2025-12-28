macro_rules! arms_contain_ref_bindings {
    () => {
        fn arms_contain_ref_bindings < 'tcx > (arms : & 'tcx [hir :: Arm < 'tcx >]) -> Option < hir :: Mutability > { arms . iter () . filter_map (| a | a . pat . contains_explicit_ref_binding ()) . max () }
    };
}

arms_contain_ref_bindings!()