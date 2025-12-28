macro_rules! deps {
    () => {
        TableLayout!();
    };
}

macro_rules! ensure_bucket_bytes_at_least_ctrl_align {
    () => {
        deps!();
        # [inline] fn ensure_bucket_bytes_at_least_ctrl_align (table_layout : TableLayout , buckets : usize) { if table_layout . size != 0 { let prod = table_layout . size . saturating_mul (buckets) ; debug_assert ! (prod >= table_layout . ctrl_align) ; } }
    };
}

ensure_bucket_bytes_at_least_ctrl_align!()