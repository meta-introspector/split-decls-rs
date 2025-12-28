macro_rules! ensure_len {
    () => {
        # [doc = " Grow a Vec by appending the type's default value until the `size` is reached."] fn ensure_len < T : Default > (v : & mut Vec < T > , size : usize) { v . resize_with (size , T :: default) ; }
    };
}

ensure_len!();