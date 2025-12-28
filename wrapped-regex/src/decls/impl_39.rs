macro_rules! deps {
    () => {
        Captures!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        # [doc = " Get a matching capture group's haystack substring by index."] # [doc = ""] # [doc = " The haystack substring returned can't outlive the `Captures` object if this"] # [doc = " method is used, because of how `Index` is defined (normally `a[i]` is part"] # [doc = " of `a` and can't outlive it). To work around this limitation, do that, use"] # [doc = " [`Captures::get`] instead."] # [doc = ""] # [doc = " `'h` is the lifetime of the matched haystack, but the lifetime of the"] # [doc = " `&str` returned by this implementation is the lifetime of the `Captures`"] # [doc = " value itself."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If there is no matching group at the given index."] impl < 'h > core :: ops :: Index < usize > for Captures < 'h > { type Output = [u8] ; fn index < 'a > (& 'a self , i : usize) -> & 'a [u8] { self . get (i) . map (| m | m . as_bytes ()) . unwrap_or_else (| | panic ! ("no group at index '{i}'")) } }
    };
}

impl_39!();