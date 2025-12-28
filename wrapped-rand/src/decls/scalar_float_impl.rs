macro_rules! deps {
    () => {
        FloatAsSIMD!();
        FloatSIMDUtils!();
        FloatSIMDScalarUtils!();
    };
}

macro_rules! scalar_float_impl {
    () => {
        deps!();
        macro_rules ! scalar_float_impl { ($ ty : ident , $ uty : ident) => { impl FloatSIMDUtils for $ ty { type Mask = bool ; type UInt = $ uty ; # [inline (always)] fn all_lt (self , other : Self) -> bool { self < other } # [inline (always)] fn all_le (self , other : Self) -> bool { self <= other } # [inline (always)] fn all_finite (self) -> bool { self . is_finite () } # [inline (always)] fn gt_mask (self , other : Self) -> Self :: Mask { self > other } # [inline (always)] fn decrease_masked (self , mask : Self :: Mask) -> Self { debug_assert ! (mask , "At least one lane must be set") ; <$ ty >:: from_bits (self . to_bits () - 1) } # [inline] fn cast_from_int (i : Self :: UInt) -> Self { i as $ ty } } # [cfg (test)] impl FloatSIMDScalarUtils for $ ty { type Scalar = $ ty ; # [inline] fn replace (self , index : usize , new_value : Self :: Scalar) -> Self { debug_assert_eq ! (index , 0) ; new_value } # [inline] fn extract_lane (self , index : usize) -> Self :: Scalar { debug_assert_eq ! (index , 0) ; self } } impl FloatAsSIMD for $ ty { } } ; }
    };
}

scalar_float_impl!();