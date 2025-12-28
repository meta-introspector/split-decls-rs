macro_rules! IdStorage {
    () => {
        # [derive (Debug , Clone)] struct IdStorage < T , # [cfg (not (feature = "std"))] S , # [cfg (feature = "std")] S = RandomState > { elements : Vec < Option < T > > , upper_bound : usize , removed_ids : IndexSet < usize , S > , }
    };
}

IdStorage!();