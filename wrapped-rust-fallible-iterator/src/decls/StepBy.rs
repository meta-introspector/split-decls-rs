macro_rules! StepBy {
    () => {
        # [doc = " An iterator which steps through the elements of the underlying iterator by a certain amount."] # [derive (Clone , Debug)] pub struct StepBy < I > { it : I , step : usize , first_take : bool , }
    };
}

StepBy!();