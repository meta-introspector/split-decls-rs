macro_rules! deps {
    () => {
        Field!();
        Positioned!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Field { # [doc = " Get the response key of the field. This is the alias if present and the"] # [doc = " name otherwise."] # [must_use] pub fn response_key (& self) -> & Positioned < Name > { self . alias . as_ref () . unwrap_or (& self . name) } # [doc = " Get the value of the argument with the specified name."] # [must_use] pub fn get_argument (& self , name : & str) -> Option < & Positioned < Value > > { self . arguments . iter () . find (| item | item . 0 . node == name) . map (| item | & item . 1) } }
    };
}

impl_15!();