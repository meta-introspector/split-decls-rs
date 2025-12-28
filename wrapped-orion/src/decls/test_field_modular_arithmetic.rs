macro_rules! deps {
    () => {
        FieldElement!();
    };
}

macro_rules! test_field_modular_arithmetic {
    () => {
        deps!();
        # [cfg (test)] mod test_field_modular_arithmetic { use super :: * ; # [test] fn test_field_ops_add () { for x in 0 .. KYBER_Q { for y in 0 .. KYBER_Q { let fe_add_ret = FieldElement (x) + FieldElement (y) ; let num_add_ret = (x + y) % KYBER_Q ; assert ! (fe_add_ret . 0 < KYBER_Q) ; assert_eq ! (fe_add_ret . 0 , num_add_ret) ; } } } # [test] fn test_field_ops_sub () { for x in 0 .. KYBER_Q { for y in 0 .. KYBER_Q { let fe_sub_ret = FieldElement (x) - FieldElement (y) ; let num_sub_ret = (x as i32 - y as i32 + KYBER_Q as i32) % KYBER_Q as i32 ; assert ! (fe_sub_ret . 0 < KYBER_Q) ; assert_eq ! (fe_sub_ret . 0 , num_sub_ret as u32) ; } } } # [test] fn test_field_ops_mul () { for x in 0 .. KYBER_Q { for y in 0 .. KYBER_Q { let fe_mul_ret = FieldElement (x) * FieldElement (y) ; let num_mul_ret = (x * y) % KYBER_Q ; assert_eq ! (fe_mul_ret . 0 , num_mul_ret) ; } } } fn conditional_sub_i16 (a : i16 , modulo : i16) -> i16 { let t : i16 = a - modulo ; let mask : i16 = t >> 15 ; (t & ! mask) | (a & mask) } # [test] fn test_conditional_sub () { for a in 0 .. KYBER_Q * 2 { if a >= KYBER_Q { assert_eq ! (conditional_sub_u32 (a) , a - KYBER_Q) ; assert_eq ! (conditional_sub_i16 (a as i16 , KYBER_Q as i16) , (a - KYBER_Q) as i16) ; } else { assert_eq ! (conditional_sub_u32 (a) , a) ; assert_eq ! (conditional_sub_i16 (a as i16 , KYBER_Q as i16) , a as i16) ; } } } # [test] fn test_field_reduced_state () { for a in 0 .. KYBER_Q . pow (2) { let reduced : u32 = barrett_reduce (a) ; assert ! ((0 .. KYBER_Q) . contains (& reduced)) ; assert_eq ! (reduced , a % KYBER_Q) ; } } }
    };
}

test_field_modular_arithmetic!()