macro_rules! deps {
    () => {
        RepeatN!();
    };
}

macro_rules! repeatn {
    () => {
        deps!();
        # [doc = " Creates a parallel iterator that produces `n` repeats of `element`"] # [doc = " (by cloning it)."] # [doc = ""] # [doc = " Deprecated in favor of [`repeat_n`] for consistency with the standard library."] # [deprecated (note = "use `repeat_n`")] pub fn repeatn < T : Clone + Send > (element : T , n : usize) -> RepeatN < T > { repeat_n (element , n) }
    };
}

repeatn!()