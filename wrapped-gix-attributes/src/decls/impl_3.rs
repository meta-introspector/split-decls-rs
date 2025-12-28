macro_rules! deps {
    () => {
        AssignmentRef!();
        Value!();
        StateRef!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl std :: fmt :: Display for AssignmentRef < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self . state { StateRef :: Set => f . write_str (self . name . as_str ()) , StateRef :: Unset => { f . write_char ('-') ? ; f . write_str (self . name . as_str ()) } StateRef :: Value (v) => { f . write_str (self . name . as_str ()) ? ; f . write_char ('=') ? ; f . write_str (v . as_bstr () . to_str_lossy () . as_ref ()) } StateRef :: Unspecified => { f . write_char ('!') ? ; f . write_str (self . name . as_str ()) } } } }
    };
}

impl_3!()