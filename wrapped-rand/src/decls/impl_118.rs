macro_rules! deps {
    () => {
        Empty!();
        Choose!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < 'a , T > Choose < 'a , T > { # [doc = " Create a new `Choose` instance which samples uniformly from the slice."] # [doc = ""] # [doc = " Returns error [`Empty`] if the slice is empty."] pub fn new (slice : & 'a [T]) -> Result < Self , Empty > { let num_choices = NonZeroUsize :: new (slice . len ()) . ok_or (Empty) ? ; Ok (Self { slice , range : UniformUsize :: new (0 , num_choices . get ()) . unwrap () , num_choices , }) } # [doc = " Returns the count of choices in this distribution"] pub fn num_choices (& self) -> NonZeroUsize { self . num_choices } }
    };
}

impl_118!();