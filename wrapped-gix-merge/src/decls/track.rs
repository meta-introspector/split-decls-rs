macro_rules! deps {
    () => {
        TrackedChange!();
        ChangeList!();
    };
}

macro_rules! track {
    () => {
        deps!();
        # [doc = " Only keep leaf nodes, or trees that are the renamed, pushing `change` on `changes`."] # [doc = " Doing so makes it easy to track renamed or rewritten or copied directories, and properly"] # [doc = " handle *their* changes that fall within them."] # [doc = " Note that it also rewrites `change` if it is a copy, turning it into an addition so copies don't have an effect"] # [doc = " on the merge algorithm."] pub fn track (change : ChangeRef < '_ > , changes : & mut ChangeList) { if change . entry_mode () . is_tree () && matches ! (change , ChangeRef :: Modification { .. }) { return ; } let is_tree = change . entry_mode () . is_tree () ; changes . push (TrackedChange { inner : match change . into_owned () { Change :: Rewrite { id , entry_mode , location , relation , copy , .. } if copy => Change :: Addition { location , relation , entry_mode , id , } , other => other , } , was_written : is_tree , needs_tree_insertion : None , rewritten_location : None , }) ; }
    };
}

track!();