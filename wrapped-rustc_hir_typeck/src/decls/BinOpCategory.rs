macro_rules! BinOpCategory {
    () => {
        # [derive (Clone , Copy)] enum BinOpCategory { # [doc = " &&, || -- cannot be overridden"] Shortcircuit , # [doc = " <<, >> -- when shifting a single integer, rhs can be any"] # [doc = " integer type. For simd, types must match."] Shift , # [doc = " +, -, etc -- takes equal types, produces same type as input,"] # [doc = " applicable to ints/floats/simd"] Math , # [doc = " &, |, ^ -- takes equal types, produces same type as input,"] # [doc = " applicable to ints/floats/simd/bool"] Bitwise , # [doc = " ==, !=, etc -- takes equal types, produces bools, except for simd,"] # [doc = " which produce the input type"] Comparison , }
    };
}

BinOpCategory!()