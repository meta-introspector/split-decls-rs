
macro_rules! remainder_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remainder in module {}", module_path!());
    };
}

mkfn!{
    remainder_introspect!();
    # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn remainder (x : f64 , y : f64) -> f64 { let (result , _) = super :: remquo (x , y) ; result }
}