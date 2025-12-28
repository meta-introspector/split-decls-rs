macro_rules! deps {
    () => {
        PropName!();
        DB!();
    };
}

macro_rules! CURRENT_SUPER_VERSION_NUMBER {
    () => {
        deps!();
        # [doc = " \"rocksdb.current-super-version-number\" - returns number of current LSM"] # [doc = " version. It is a uint64_t integer number, incremented after there is"] # [doc = " any change to the LSM tree. The number is not preserved after restarting"] # [doc = " the DB. After DB restart, it will start from 0 again."] pub const CURRENT_SUPER_VERSION_NUMBER : & PropName = property ! ("current-super-version-number") ;
    };
}

CURRENT_SUPER_VERSION_NUMBER!();