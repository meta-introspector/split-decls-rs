macro_rules! deps {
    () => {
        Events!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl Events { # [doc = " Creates a new blank event bit mask."] pub fn new () -> Events { Events { bits : 0 } } # [doc = " Set or unset the whether these events indicate that input is ready."] pub fn input (& mut self , val : bool) -> & mut Events { self . flag (curl_sys :: CURL_CSELECT_IN , val) } # [doc = " Set or unset the whether these events indicate that output is ready."] pub fn output (& mut self , val : bool) -> & mut Events { self . flag (curl_sys :: CURL_CSELECT_OUT , val) } # [doc = " Set or unset the whether these events indicate that an error has"] # [doc = " happened."] pub fn error (& mut self , val : bool) -> & mut Events { self . flag (curl_sys :: CURL_CSELECT_ERR , val) } fn flag (& mut self , flag : c_int , val : bool) -> & mut Events { if val { self . bits |= flag ; } else { self . bits &= ! flag ; } self } }
    };
}

impl_144!();