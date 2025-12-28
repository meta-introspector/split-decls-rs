macro_rules! deps {
    () => {
        DateTime!();
        MappedLocalTime!();
        TimeZone!();
        Utc!();
        FixedOffset!();
    };
}

macro_rules! impl_676 {
    () => {
        deps!();
        impl < T : fmt :: Debug > MappedLocalTime < T > { # [doc = " Returns a single unique conversion result or panics."] # [doc = ""] # [doc = " `unwrap()` is best combined with time zone types where the mapping can never fail like"] # [doc = " [`Utc`] and [`FixedOffset`]. Note that for [`FixedOffset`] there is a rare case where a"] # [doc = " resulting [`DateTime`] can be out of range."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the local time falls within a _fold_ or a _gap_ in the local time, and on any"] # [doc = " error that may have been returned by the type implementing [`TimeZone`]."] # [must_use] # [track_caller] pub fn unwrap (self) -> T { match self { MappedLocalTime :: None => panic ! ("No such local time") , MappedLocalTime :: Single (t) => t , MappedLocalTime :: Ambiguous (t1 , t2) => { panic ! ("Ambiguous local time, ranging from {t1:?} to {t2:?}") } } } }
    };
}

impl_676!()