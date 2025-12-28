macro_rules! normalize_scalar {
    () => {
        # [inline (always)] fn normalize_scalar (positions : & mut [i32] , norm_offset : i32) { positions . iter_mut () . for_each (| p | * p = p . saturating_sub (norm_offset)) ; }
    };
}

normalize_scalar!();