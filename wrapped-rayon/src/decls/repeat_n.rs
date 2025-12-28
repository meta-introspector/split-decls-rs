macro_rules! deps {
    () => {
        RepeatN!();
        RepeatNProducer!();
        Empty!();
    };
}

macro_rules! repeat_n {
    () => {
        deps!();
        # [doc = " Creates a parallel iterator that produces `n` repeats of `element`"] # [doc = " (by cloning it)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rayon::prelude::*;"] # [doc = " use rayon::iter::repeat_n;"] # [doc = " let x: Vec<(i32, i32)> = repeat_n(22, 3).zip(0..3).collect();"] # [doc = " assert_eq!(x, vec![(22, 0), (22, 1), (22, 2)]);"] # [doc = " ```"] pub fn repeat_n < T : Clone + Send > (element : T , n : usize) -> RepeatN < T > { let inner = match NonZeroUsize :: new (n) { Some (count) => RepeatNProducer :: Repeats (element , count) , None => RepeatNProducer :: Empty , } ; RepeatN { inner } }
    };
}

repeat_n!();