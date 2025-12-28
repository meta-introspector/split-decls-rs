macro_rules! Closed {
    () => {
        # [doc = " Marker to signal the Registration is a closed file that consumes no additional process resources."] # [doc = ""] # [doc = " It can't ever be written to unless reopened after persisting it."] # [derive (Debug)] pub struct Closed ;
    };
}

Closed!()