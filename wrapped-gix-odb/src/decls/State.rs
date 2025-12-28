macro_rules! deps {
    () => {
        EntryForOrdering!();
        Iter!();
        IndexLookup!();
    };
}

macro_rules! State {
    () => {
        deps!();
        enum State { Pack { index_iter : IntoIter < handle :: IndexLookup > , index : handle :: IndexLookup , ordered_entries : Option < Vec < EntryForOrdering > > , entry_index : u32 , num_objects : u32 , } , Loose { iter : loose :: Iter , index : usize , } , Depleted , }
    };
}

State!()