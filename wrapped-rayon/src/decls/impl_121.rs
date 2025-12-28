macro_rules! deps {
    () => {
        Splitter!();
        LengthSplitter!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl LengthSplitter { # [doc = " Creates a new splitter based on lengths."] # [doc = ""] # [doc = " The `min` is a hard lower bound.  We'll never split below that, but"] # [doc = " of course an iterator might start out smaller already."] # [doc = ""] # [doc = " The `max` is an upper bound on the working size, used to determine"] # [doc = " the minimum number of times we need to split to get under that limit."] # [doc = " The adaptive algorithm may very well split even further, but never"] # [doc = " smaller than the `min`."] # [inline] fn new (min : usize , max : usize , len : usize) -> LengthSplitter { let mut splitter = LengthSplitter { inner : Splitter :: new () , min : Ord :: max (min , 1) , } ; let min_splits = len / Ord :: max (max , 1) ; if min_splits > splitter . inner . splits { splitter . inner . splits = min_splits ; } splitter } # [inline] fn try_split (& mut self , len : usize , stolen : bool) -> bool { len / 2 >= self . min && self . inner . try_split (stolen) } }
    };
}

impl_121!();