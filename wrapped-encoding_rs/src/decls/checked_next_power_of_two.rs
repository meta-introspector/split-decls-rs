macro_rules! checked_next_power_of_two {
    () => {
        # [cfg (feature = "alloc")] # [inline (always)] fn checked_next_power_of_two (opt : Option < usize >) -> Option < usize > { opt . map (| n | n . next_power_of_two ()) }
    };
}

checked_next_power_of_two!();