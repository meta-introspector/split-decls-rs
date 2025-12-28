macro_rules! IntoFloat {
    () => {
        # [doc (hidden)] pub trait IntoFloat { type F ; # [doc = " Helper method to combine the fraction and a constant exponent into a"] # [doc = " float."] # [doc = ""] # [doc = " Only the least significant bits of `self` may be set, 23 for `f32` and"] # [doc = " 52 for `f64`."] # [doc = " The resulting value will fall in a range that depends on the exponent."] # [doc = " As an example the range with exponent 0 will be"] # [doc = " [2<sup>0</sup>..2<sup>1</sup>), which is [1..2)."] fn into_float_with_exponent (self , exponent : i32) -> Self :: F ; }
    };
}

IntoFloat!();