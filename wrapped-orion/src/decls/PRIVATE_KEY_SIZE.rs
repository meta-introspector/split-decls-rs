macro_rules! deps {
    () => {
        DecapsulationKey!();
    };
}

macro_rules! PRIVATE_KEY_SIZE {
    () => {
        deps!();
        # [doc = " Size of private [DecapsulationKey]."] pub const PRIVATE_KEY_SIZE : usize = 32 ;
    };
}

PRIVATE_KEY_SIZE!()