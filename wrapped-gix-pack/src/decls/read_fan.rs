macro_rules! read_fan {
    () => {
        fn read_fan (d : & [u8]) -> ([u32 ; FAN_LEN] , usize) { assert ! (d . len () >= FAN_LEN * N32_SIZE) ; let mut fan = [0 ; FAN_LEN] ; for (c , f) in d . chunks_exact (N32_SIZE) . zip (fan . iter_mut ()) { * f = crate :: read_u32 (c) ; } (fan , FAN_LEN * N32_SIZE) }
    };
}

read_fan!();