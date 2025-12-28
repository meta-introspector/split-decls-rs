macro_rules! deps {
    () => {
        TrackedChange!();
        ChangeList!();
    };
}

macro_rules! push_deferred_with_rewrite {
    () => {
        deps!();
        fn push_deferred_with_rewrite ((change , ours_idx) : (Change , Option < usize >) , new_location : Option < (BString , usize) > , changes : & mut ChangeList ,) { changes . push (TrackedChange { inner : change , was_written : false , needs_tree_insertion : Some (ours_idx) , rewritten_location : new_location , }) ; }
    };
}

push_deferred_with_rewrite!();