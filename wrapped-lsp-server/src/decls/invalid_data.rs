macro_rules! invalid_data {
    () => {
        macro_rules ! invalid_data { ($ ($ tt : tt) *) => (invalid_data (format ! ($ ($ tt) *))) }
    };
}

invalid_data!()