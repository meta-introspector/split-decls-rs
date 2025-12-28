macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! ManyMN {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [doc = " Parser implementation for the [many_m_n] combinator"] pub struct ManyMN < F > { parser : F , min : usize , max : usize , }
    };
}

ManyMN!()