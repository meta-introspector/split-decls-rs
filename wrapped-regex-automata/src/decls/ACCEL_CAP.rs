macro_rules! ACCEL_CAP {
    () => {
        # [doc = " The capacity of each accelerator, in bytes. We set this to 8 since it's a"] # [doc = " multiple of 4 (our ID size) and because it gives us a little wiggle room"] # [doc = " if we want to support more accel bytes in the future without a breaking"] # [doc = " change."] # [doc = ""] # [doc = " This MUST be a multiple of ACCEL_TY_SIZE."] const ACCEL_CAP : usize = 8 ;
    };
}

ACCEL_CAP!();