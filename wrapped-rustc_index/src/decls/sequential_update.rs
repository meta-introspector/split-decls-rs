macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! sequential_update {
    () => {
        deps!();
        fn sequential_update < T : Idx > (mut self_update : impl FnMut (T) -> bool , it : impl Iterator < Item = T > ,) -> bool { it . fold (false , | changed , elem | self_update (elem) | changed) }
    };
}

sequential_update!();