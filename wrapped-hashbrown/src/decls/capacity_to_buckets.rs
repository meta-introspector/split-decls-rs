macro_rules! deps {
    () => {
        TableLayout!();
    };
}

macro_rules! capacity_to_buckets {
    () => {
        deps!();
        # [doc = " Returns the number of buckets needed to hold the given number of items,"] # [doc = " taking the maximum load factor into account."] # [doc = ""] # [doc = " Returns `None` if an overflow occurs."] # [doc = ""] # [doc = " This ensures that `buckets * table_layout.size >= table_layout.ctrl_align`."] # [cfg_attr (target_os = "emscripten" , inline (never))] # [cfg_attr (not (target_os = "emscripten") , inline)] fn capacity_to_buckets (cap : usize , table_layout : TableLayout) -> Option < usize > { debug_assert_ne ! (cap , 0) ; if cap < 15 { let min_cap = match (Group :: WIDTH , table_layout . size) { (16 , 0 ..= 1) => 14 , (16 , 2 ..= 3) => 7 , (8 , 0 ..= 1) => 7 , _ => 3 , } ; let cap = min_cap . max (cap) ; let buckets = if cap < 4 { 4 } else if cap < 8 { 8 } else { 16 } ; ensure_bucket_bytes_at_least_ctrl_align (table_layout , buckets) ; return Some (buckets) ; } let adjusted_cap = cap . checked_mul (8) ? / 7 ; let buckets = adjusted_cap . next_power_of_two () ; ensure_bucket_bytes_at_least_ctrl_align (table_layout , buckets) ; Some (buckets) }
    };
}

capacity_to_buckets!();