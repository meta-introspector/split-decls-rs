macro_rules! deps {
    () => {
        Either!();
        NestedProgress!();
        DoOrDiscard!();
        Discard!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < T : NestedProgress > DoOrDiscard < T > { # [doc = " Obtain either the original [`NestedProgress`] implementation or `None`."] pub fn into_inner (self) -> Option < T > { match self { DoOrDiscard (Either :: Left (p)) => Some (p) , DoOrDiscard (Either :: Right (_)) => None , } } # [doc = " Take out the implementation of [`NestedProgress`] and replace it with [`Discard`]."] pub fn take (& mut self) -> Option < T > { let this = std :: mem :: replace (self , DoOrDiscard :: from (None)) ; match this { DoOrDiscard (Either :: Left (p)) => Some (p) , DoOrDiscard (Either :: Right (_)) => None , } } }
    };
}

impl_165!()