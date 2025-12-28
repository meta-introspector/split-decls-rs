macro_rules! FloatSIMDUtils {
    () => {
        # [doc = " Helper trait when dealing with scalar and SIMD floating point types."] pub (crate) trait FloatSIMDUtils { fn all_lt (self , other : Self) -> bool ; fn all_le (self , other : Self) -> bool ; fn all_finite (self) -> bool ; type Mask ; fn gt_mask (self , other : Self) -> Self :: Mask ; fn decrease_masked (self , mask : Self :: Mask) -> Self ; type UInt ; fn cast_from_int (i : Self :: UInt) -> Self ; }
    };
}

FloatSIMDUtils!()