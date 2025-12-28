macro_rules! deps {
    () => {
        AccelTy!();
    };
}

macro_rules! ACCEL_TY_SIZE {
    () => {
        deps!();
        # [doc = " The size of the unit of representation for accelerators."] # [doc = ""] # [doc = " ACCEL_CAP *must* be a multiple of this size."] const ACCEL_TY_SIZE : usize = core :: mem :: size_of :: < AccelTy > () ;
    };
}

ACCEL_TY_SIZE!();