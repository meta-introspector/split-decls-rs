macro_rules! deps {
    () => {
        TryMaybeDone!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < Fut : TryFuture > TryMaybeDone < Fut > { # [doc = " Returns an [`Option`] containing a mutable reference to the output of the future."] # [doc = " The output of this method will be [`Some`] if and only if the inner"] # [doc = " future has completed successfully and [`take_output`](TryMaybeDone::take_output)"] # [doc = " has not yet been called."] # [inline] pub fn output_mut (self : Pin < & mut Self >) -> Option < & mut Fut :: Ok > { unsafe { match self . get_unchecked_mut () { Self :: Done (res) => Some (res) , _ => None , } } } # [doc = " Attempt to take the output of a `TryMaybeDone` without driving it"] # [doc = " towards completion."] # [inline] pub fn take_output (self : Pin < & mut Self >) -> Option < Fut :: Ok > { match & * self { Self :: Done (_) => { } Self :: Future (_) | Self :: Gone => return None , } unsafe { match mem :: replace (self . get_unchecked_mut () , Self :: Gone) { Self :: Done (output) => Some (output) , _ => unreachable ! () , } } } }
    };
}

impl_169!();