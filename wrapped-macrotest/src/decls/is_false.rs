macro_rules! is_false {
    () => {
        # [allow (clippy :: trivially_copy_pass_by_ref)] fn is_false (boolean : & bool) -> bool { ! * boolean }
    };
}

is_false!();