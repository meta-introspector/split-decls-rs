macro_rules! deps {
    () => {
        RepeatNProducer!();
    };
}

macro_rules! RepeatN {
    () => {
        deps!();
        # [doc = " Iterator adaptor for [the `repeat_n()` function]."] # [doc = ""] # [doc = " [the `repeat_n()` function]: repeat_n()"] # [derive (Clone)] pub struct RepeatN < T > { inner : RepeatNProducer < T > , }
    };
}

RepeatN!()