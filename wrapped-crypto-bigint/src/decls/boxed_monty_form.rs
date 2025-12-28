macro_rules! boxed_monty_form {
    () => {
        # [cfg (feature = "alloc")] pub (crate) mod boxed_monty_form ;
    };
}

boxed_monty_form!();