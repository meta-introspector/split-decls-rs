macro_rules! deps {
    () => {
        Accel!();
    };
}

macro_rules! ACCEL_LEN {
    () => {
        deps!();
        # [doc = " The maximum length in bytes that a single Accel can be. This is distinct"] # [doc = " from the capacity of an accelerator in that the length represents only the"] # [doc = " bytes that should be read."] const ACCEL_LEN : usize = 4 ;
    };
}

ACCEL_LEN!()