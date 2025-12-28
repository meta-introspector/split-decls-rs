macro_rules! deps {
    () => {
        RepeatN!();
    };
}

macro_rules! repeat_n {
    () => {
        deps!();
        # [doc = " Create an iterator that produces `n` repetitions of `element`."] pub fn repeat_n < A > (element : A , n : usize) -> RepeatN < A > where A : Clone , { if n == 0 { RepeatN { elt : None , n } } else { RepeatN { elt : Some (element) , n , } } }
    };
}

repeat_n!()