macro_rules! counter_high {
    () => {
        # [inline] fn counter_high (counter : u64) -> u32 { (counter >> 32) as u32 }
    };
}

counter_high!()