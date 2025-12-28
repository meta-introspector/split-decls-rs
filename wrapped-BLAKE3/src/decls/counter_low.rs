macro_rules! counter_low {
    () => {
        # [inline] fn counter_low (counter : u64) -> u32 { counter as u32 }
    };
}

counter_low!();