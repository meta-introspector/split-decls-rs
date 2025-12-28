macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < C > DebugWithContext < C > for State { fn fmt_with (& self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("qualif: ") ? ; self . qualif . fmt_with (ctxt , f) ? ; f . write_str (" borrow: ") ? ; self . borrow . fmt_with (ctxt , f) ? ; Ok (()) } fn fmt_diff_with (& self , old : & Self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self == old { return Ok (()) ; } if self . qualif != old . qualif { f . write_str ("qualif: ") ? ; self . qualif . fmt_diff_with (& old . qualif , ctxt , f) ? ; f . write_str ("\n") ? ; } if self . borrow != old . borrow { f . write_str ("borrow: ") ? ; self . qualif . fmt_diff_with (& old . borrow , ctxt , f) ? ; f . write_str ("\n") ? ; } Ok (()) } }
    };
}

impl_73!();