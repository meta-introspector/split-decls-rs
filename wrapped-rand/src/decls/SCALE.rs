macro_rules! SCALE {
    () => {
        const SCALE : f64 = 2.0 * (1u64 << 63) as f64 ;
    };
}

SCALE!();