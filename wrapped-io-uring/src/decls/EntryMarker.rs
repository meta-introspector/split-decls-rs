macro_rules! deps {
    () => {
        Entry128!();
        Entry!();
    };
}

macro_rules! EntryMarker {
    () => {
        deps!();
        # [doc = " A submission queue entry (SQE), representing a request for an I/O operation."] # [doc = ""] # [doc = " This is implemented for [`Entry`] and [`Entry128`]."] pub trait EntryMarker : Clone + Debug + From < Entry > + private :: Sealed { const BUILD_FLAGS : u32 ; }
    };
}

EntryMarker!()