macro_rules! IndexSet {
    () => {
        # [cfg (not (feature = "write_std"))] type IndexSet < K > = indexmap :: IndexSet < K , hashbrown :: DefaultHashBuilder > ;
    };
}

IndexSet!()