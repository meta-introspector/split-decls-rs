
macro_rules! test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test in module {}", module_path!());
    };
}

mkfn!{
    test_introspect!();
    # [doc = " Tests the `bit` of `x`."] # [allow (dead_code)] # [inline] pub (crate) fn test (x : usize , bit : u32) -> bool { debug_assert ! (bit < usize :: BITS , "bit index out-of-bounds") ; x & (1 << bit) != 0 }
}