macro_rules! deps {
    () => {
        Counter!();
        Sender!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < C > Sender < C > { # [doc = " Returns the internal `Counter`."] fn counter (& self) -> & Counter < C > { unsafe { self . counter . as_ref () } } # [doc = " Acquires another sender reference."] pub (crate) fn acquire (& self) -> Self { let count = self . counter () . senders . fetch_add (1 , Ordering :: Relaxed) ; if count > isize :: MAX as usize { process :: abort () ; } Self { counter : self . counter , } } # [doc = " Releases the sender reference."] # [doc = ""] # [doc = " Function `disconnect` will be called if this is the last sender reference."] pub (crate) unsafe fn release < F : FnOnce (& C) -> bool > (& self , disconnect : F) { if self . counter () . senders . fetch_sub (1 , Ordering :: AcqRel) == 1 { disconnect (& self . counter () . chan) ; if self . counter () . destroy . swap (true , Ordering :: AcqRel) { drop (unsafe { Box :: from_raw (self . counter . as_ptr ()) }) ; } } } }
    };
}

impl_52!()