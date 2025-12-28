macro_rules! singular_or_plural {
    () => {
        # [doc = " Returns the singular or plural form on the verb to be based on the argument's value."] fn singular_or_plural (n : usize) -> & 'static str { if n > 1 { " were provided" } else { " was provided" } }
    };
}

singular_or_plural!()