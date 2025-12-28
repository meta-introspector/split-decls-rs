macro_rules! DepGraphDataMerger {
    () => {
        pub trait DepGraphDataMerger { fn merge_data (& self , layer_data : HashMap < String , i32 > , usage_counts : HashMap < String , u32 > ,) -> Result < HashMap < String , MergedCrateInfo > > ; }
    };
}

DepGraphDataMerger!()