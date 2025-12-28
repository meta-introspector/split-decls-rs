macro_rules! deps {
    () => {
        PropertyName!();
    };
}

macro_rules! aggregated_table_properties_at_level {
    () => {
        deps!();
        # [doc = " \"rocksdb.aggregated-table-properties-at-`level<N>`\", same as the previous"] # [doc = " one but only returns the aggregated table properties of the"] # [doc = " specified level \"N\" at the target column family."] pub fn aggregated_table_properties_at_level (level : usize) -> PropertyName { unsafe { level_property ("aggregated-table-properties-at-level" , level) } }
    };
}

aggregated_table_properties_at_level!()