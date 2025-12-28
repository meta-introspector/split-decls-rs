macro_rules! deps {
    () => {
        FutureArray!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T , const N : usize > FutureArray < T , N > { # [doc = " Create a new instance of `FutureArray`"] pub (crate) fn new (futures : [T ; N]) -> Self { let futures = MaybeUninit :: new (futures) ; let futures = unsafe { mem :: transmute_copy (& mem :: ManuallyDrop :: new (futures)) } ; Self { futures } } # [doc = " Create an iterator of pinned references."] pub (crate) fn iter (self : Pin < & mut Self >) -> impl Iterator < Item = Pin < & mut ManuallyDrop < T > > > { unsafe { self . get_unchecked_mut () } . futures . iter_mut () . map (| t | unsafe { Pin :: new_unchecked (t) }) } # [doc = " Drop a future at the given index."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The future is held in a `ManuallyDrop`, so no double-dropping, etc"] pub (crate) unsafe fn drop (mut self : Pin < & mut Self > , idx : usize) { unsafe { let futures = self . as_mut () . get_unchecked_mut () . futures . as_mut () ; ManuallyDrop :: drop (& mut futures [idx]) ; } ; } }
    };
}

impl_10!();