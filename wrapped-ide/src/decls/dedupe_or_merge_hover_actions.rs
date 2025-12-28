macro_rules! deps {
    () => {
        Runnable!();
        HoverAction!();
    };
}

macro_rules! dedupe_or_merge_hover_actions {
    () => {
        deps!();
        fn dedupe_or_merge_hover_actions (actions : Vec < HoverAction >) -> Vec < HoverAction > { let mut deduped_actions = Vec :: with_capacity (actions . len ()) ; let mut go_to_type_targets = FxIndexSet :: default () ; let mut seen_implementation = false ; let mut seen_reference = false ; let mut seen_runnable = false ; for action in actions { match action { HoverAction :: GoToType (targets) => { go_to_type_targets . extend (targets) ; } HoverAction :: Implementation (..) => { if ! seen_implementation { seen_implementation = true ; deduped_actions . push (action) ; } } HoverAction :: Reference (..) => { if ! seen_reference { seen_reference = true ; deduped_actions . push (action) ; } } HoverAction :: Runnable (..) => { if ! seen_runnable { seen_runnable = true ; deduped_actions . push (action) ; } } } ; } if ! go_to_type_targets . is_empty () { deduped_actions . push (HoverAction :: GoToType (go_to_type_targets . into_iter () . sorted_by (| a , b | a . mod_path . cmp (& b . mod_path)) . collect () ,)) ; } deduped_actions }
    };
}

dedupe_or_merge_hover_actions!();