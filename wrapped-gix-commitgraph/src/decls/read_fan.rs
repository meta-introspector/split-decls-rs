macro_rules! read_fan {
    () => {
        fn read_fan (d : & [u8]) -> ([u32 ; FAN_LEN] , usize) { assert ! (d . len () >= FAN_LEN * 4) ; let mut fan = [0 ; FAN_LEN] ; for (c , f) in d . chunks_exact (4) . zip (fan . iter_mut ()) { * f = u32 :: from_be_bytes (c . try_into () . unwrap ()) ; } (fan , FAN_LEN * 4) }
    };
}

read_fan!();