macro_rules! deps {
    () => {
        CombinationsGeneric!();
        LazyBuffer!();
        PoolIndex!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < I : Iterator , Idx : PoolIndex < I :: Item > > CombinationsGeneric < I , Idx > { # [doc = " Constructor with arguments the inner iterator and the initial state for the indices."] fn new (iter : I , indices : Idx) -> Self { Self { indices , pool : LazyBuffer :: new (iter) , first : true , } } # [doc = " Returns the length of a combination produced by this iterator."] # [inline] pub fn k (& self) -> usize { self . indices . len () } # [doc = " Returns the (current) length of the pool from which combination elements are"] # [doc = " selected. This value can change between invocations of [`next`](Combinations::next)."] # [inline] pub fn n (& self) -> usize { self . pool . len () } # [doc = " Returns a reference to the source pool."] # [inline] pub (crate) fn src (& self) -> & LazyBuffer < I > { & self . pool } # [doc = " Return the length of the inner iterator and the count of remaining combinations."] pub (crate) fn n_and_count (self) -> (usize , usize) { let Self { indices , pool , first , } = self ; let n = pool . count () ; (n , remaining_for (n , first , indices . borrow ()) . unwrap ()) } # [doc = " Initialises the iterator by filling a buffer with elements from the"] # [doc = " iterator. Returns true if there are no combinations, false otherwise."] fn init (& mut self) -> bool { self . pool . prefill (self . k ()) ; let done = self . k () > self . n () ; if ! done { self . first = false ; } done } # [doc = " Increments indices representing the combination to advance to the next"] # [doc = " (in lexicographic order by increasing sequence) combination. For example"] # [doc = " if we have n=4 & k=2 then `[0, 1] -> [0, 2] -> [0, 3] -> [1, 2] -> ...`"] # [doc = ""] # [doc = " Returns true if we've run out of combinations, false otherwise."] fn increment_indices (& mut self) -> bool { let indices = self . indices . borrow_mut () ; if indices . is_empty () { return true ; } let mut i : usize = indices . len () - 1 ; if indices [i] == self . pool . len () - 1 { self . pool . get_next () ; } while indices [i] == i + self . pool . len () - indices . len () { if i > 0 { i -= 1 ; } else { return true ; } } indices [i] += 1 ; for j in i + 1 .. indices . len () { indices [j] = indices [j - 1] + 1 ; } false } # [doc = " Returns the n-th item or the number of successful steps."] pub (crate) fn try_nth (& mut self , n : usize) -> Result < < Self as Iterator > :: Item , usize > where I : Iterator , I :: Item : Clone , { let done = if self . first { self . init () } else { self . increment_indices () } ; if done { return Err (0) ; } for i in 0 .. n { if self . increment_indices () { return Err (i + 1) ; } } Ok (self . indices . extract_item (& self . pool)) } }
    };
}

impl_181!()