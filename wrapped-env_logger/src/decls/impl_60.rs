macro_rules! deps {
    () => {
        TimestampPrecision!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        # [doc = " The default timestamp precision is seconds."] impl Default for TimestampPrecision { fn default () -> Self { TimestampPrecision :: Seconds } }
    };
}

impl_60!();