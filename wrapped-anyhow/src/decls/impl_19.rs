macro_rules! deps {
    () => {
        Chain!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl ExactSizeIterator for Chain < '_ > { fn len (& self) -> usize { match & self . state { Linked { mut next } => { let mut len = 0 ; while let Some (cause) = next { next = cause . source () ; len += 1 ; } len } # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] Buffered { rest } => rest . len () , } } }
    };
}

impl_19!()