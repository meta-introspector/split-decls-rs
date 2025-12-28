macro_rules! size_t {
    () => {
        # [allow (non_camel_case_types)] pub type size_t = usize ;
    };
}

size_t!();