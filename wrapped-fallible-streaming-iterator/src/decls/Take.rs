macro_rules! Take {
    () => {
        # [doc = " An iterator which only returns a number of initial elements."] pub struct Take < I > { it : I , n : usize , done : bool , }
    };
}

Take!()