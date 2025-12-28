macro_rules! deps {
    () => {
        DoubleFloat!();
    };
}

macro_rules! DoubleDouble {
    () => {
        deps!();
        # [doc = " 128-bit floating point number comprised of two IEEE [`Double`](ieee::Double) values."] # [doc = ""] # [doc = " This is the \"IBM Extended Double\" format, described at"] # [doc = " <https://www.ibm.com/docs/en/aix/7.3?topic=sepl-128-bit-long-double-floating-point-data-type>."] pub type DoubleDouble = DoubleFloat < ieee :: Double > ;
    };
}

DoubleDouble!();