macro_rules! deps {
    () => {
        Tuple!();
        Distributions!();
    };
}

macro_rules! TupledDistributionsBuilder {
    () => {
        deps!();
        # [doc = " A tuple of vecs used to build distributions."] pub trait TupledDistributionsBuilder : Sized { # [doc = " A tuple that can be pushed/inserted into the tupled distributions"] type Item : Tuple < Builder = Self > ; # [doc = " Creates a new tuple of vecs"] fn new (size : usize) -> Self ; # [doc = " Push one element into each of the vecs"] fn push (& mut self , tuple : Self :: Item) ; # [doc = " Append one tuple of vecs to this one, leaving the vecs in the other tuple empty"] fn extend (& mut self , other : & mut Self) ; # [doc = " Convert the tuple of vectors into a tuple of distributions"] fn complete (self) -> < Self :: Item as Tuple > :: Distributions ; }
    };
}

TupledDistributionsBuilder!()