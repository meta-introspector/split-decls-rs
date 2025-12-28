macro_rules! Skip {
    () => {
        # [doc = " An iterator which skips a number of initial elements."] pub struct Skip < I > { it : I , n : usize , }
    };
}

Skip!()