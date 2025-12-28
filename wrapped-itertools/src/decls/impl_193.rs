macro_rules! deps {
    () => {
        PoolIndex!();
        CombinationsWithReplacementGeneric!();
        LazyBuffer!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < I : Iterator , Idx : PoolIndex < I :: Item > > CombinationsWithReplacementGeneric < I , Idx > where I : Iterator , I :: Item : Clone , { # [doc = " Increments indices representing the combination to advance to the next"] # [doc = " (in lexicographic order by increasing sequence) combination."] # [doc = ""] # [doc = " Returns true if we've run out of combinations, false otherwise."] fn increment_indices (& mut self) -> bool { self . pool . get_next () ; let mut increment = None ; let indices : & mut [usize] = self . indices . borrow_mut () ; for (i , indices_int) in indices . iter () . enumerate () . rev () { if * indices_int < self . pool . len () - 1 { increment = Some ((i , indices_int + 1)) ; break ; } } match increment { Some ((increment_from , increment_value)) => { indices [increment_from ..] . fill (increment_value) ; false } None => true , } } # [doc = " Constructor with arguments the inner iterator and the initial state for the indices."] fn new (iter : I , indices : Idx) -> Self { Self { indices , pool : LazyBuffer :: new (iter) , first : true , } } }
    };
}

impl_193!()