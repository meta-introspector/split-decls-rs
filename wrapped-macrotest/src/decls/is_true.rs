macro_rules! is_true {
    () => {
        # [allow (clippy :: trivially_copy_pass_by_ref)] fn is_true (boolean : & bool) -> bool { * boolean }
    };
}

is_true!();