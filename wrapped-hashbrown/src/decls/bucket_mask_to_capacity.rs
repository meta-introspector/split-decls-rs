macro_rules! bucket_mask_to_capacity {
    () => {
        # [doc = " Returns the maximum effective capacity for the given bucket mask, taking"] # [doc = " the maximum load factor into account."] # [inline] fn bucket_mask_to_capacity (bucket_mask : usize) -> usize { if bucket_mask < 8 { bucket_mask } else { ((bucket_mask + 1) / 8) * 7 } }
    };
}

bucket_mask_to_capacity!()