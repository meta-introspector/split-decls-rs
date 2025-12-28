macro_rules! ldexpf {
    () => {
        # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ldexpf (x : f32 , n : i32) -> f32 { super :: scalbnf (x , n) }
    };
}

ldexpf!()