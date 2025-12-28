macro_rules! Skip {
    () => {
        # [doc = " An iterator which skips initial elements."] # [derive (Clone , Debug)] pub struct Skip < I > { it : I , n : usize , }
    };
}

Skip!()