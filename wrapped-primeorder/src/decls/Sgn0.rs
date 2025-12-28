macro_rules! Sgn0 {
    () => {
        # [doc = " Trait for determining the parity of the field"] pub trait Sgn0 { # [doc = " Return the parity of the field"] # [doc = " 1 == negative"] # [doc = " 0 == non-negative"] fn sgn0 (& self) -> Choice ; }
    };
}

Sgn0!()