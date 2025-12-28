macro_rules! deps {
    () => {
        BorrowckDomain!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'tcx , C > DebugWithContext < C > for BorrowckDomain where C : rustc_mir_dataflow :: move_paths :: HasMoveData < 'tcx > , { fn fmt_with (& self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("borrows: ") ? ; self . borrows . fmt_with (ctxt , f) ? ; f . write_str (" uninits: ") ? ; self . uninits . fmt_with (ctxt , f) ? ; f . write_str (" ever_inits: ") ? ; self . ever_inits . fmt_with (ctxt , f) ? ; Ok (()) } fn fmt_diff_with (& self , old : & Self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self == old { return Ok (()) ; } if self . borrows != old . borrows { f . write_str ("borrows: ") ? ; self . borrows . fmt_diff_with (& old . borrows , ctxt , f) ? ; f . write_str ("\n") ? ; } if self . uninits != old . uninits { f . write_str ("uninits: ") ? ; self . uninits . fmt_diff_with (& old . uninits , ctxt , f) ? ; f . write_str ("\n") ? ; } if self . ever_inits != old . ever_inits { f . write_str ("ever_inits: ") ? ; self . ever_inits . fmt_diff_with (& old . ever_inits , ctxt , f) ? ; f . write_str ("\n") ? ; } Ok (()) } }
    };
}

impl_48!()