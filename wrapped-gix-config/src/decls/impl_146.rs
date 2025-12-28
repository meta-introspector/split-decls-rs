macro_rules! deps {
    () => {
        Comment!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl Comment < '_ > { # [doc = " Turn this instance into a fully owned one with `'static` lifetime."] # [must_use] pub fn to_owned (& self) -> Comment < 'static > { Comment { tag : self . tag , text : Cow :: Owned (self . text . as_ref () . into ()) , } } # [doc = " Serialize this type into a `BString` for convenience."] # [doc = ""] # [doc = " Note that `to_string()` can also be used, but might not be lossless."] # [must_use] pub fn to_bstring (& self) -> BString { let mut buf = Vec :: new () ; self . write_to (& mut buf) . expect ("io error impossible") ; buf . into () } # [doc = " Stream ourselves to the given `out`, in order to reproduce this comment losslessly."] pub fn write_to (& self , mut out : impl std :: io :: Write) -> std :: io :: Result < () > { out . write_all (& [self . tag]) ? ; out . write_all (self . text . as_ref ()) } }
    };
}

impl_146!();