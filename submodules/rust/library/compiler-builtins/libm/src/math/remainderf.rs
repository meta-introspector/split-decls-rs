
macro_rules! remainderf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remainderf in module {}", module_path!());
    };
}

mkfn!{
    remainderf_introspect!();
    # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn remainderf (x : f32 , y : f32) -> f32 { let (result , _) = super :: remquof (x , y) ; result }
}