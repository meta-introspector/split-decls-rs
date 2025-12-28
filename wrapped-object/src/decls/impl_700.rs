macro_rules! deps {
    () => {
        ImportThunkList!();
        ImageThunkData!();
        ImageNtHeaders!();
        Result!();
    };
}

macro_rules! impl_700 {
    () => {
        deps!();
        impl < 'data > ImportThunkList < 'data > { # [doc = " Get the thunk at the given index."] pub fn get < Pe : ImageNtHeaders > (& self , index : usize) -> Result < Pe :: ImageThunkData > { let thunk = self . data . read_at (index * mem :: size_of :: < Pe :: ImageThunkData > ()) . read_error ("Invalid PE import thunk index") ? ; Ok (* thunk) } # [doc = " Return the first thunk in the list, and update `self` to point after it."] # [doc = ""] # [doc = " Returns `Ok(None)` when a null thunk is found."] pub fn next < Pe : ImageNtHeaders > (& mut self) -> Result < Option < Pe :: ImageThunkData > > { let thunk = self . data . read :: < Pe :: ImageThunkData > () . read_error ("Missing PE null import thunk") ? ; if thunk . address () == 0 { Ok (None) } else { Ok (Some (* thunk)) } } }
    };
}

impl_700!()