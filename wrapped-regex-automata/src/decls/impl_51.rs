macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for State < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for (i , (start , end , sid)) in self . sparse_transitions () . enumerate () { let id = if f . alternate () { sid . as_usize () } else { sid . as_usize () >> self . stride2 } ; if i > 0 { write ! (f , ", ") ? ; } if start == end { write ! (f , "{start:?} => {id:?}") ? ; } else { write ! (f , "{start:?}-{end:?} => {id:?}") ? ; } } Ok (()) } }
    };
}

impl_51!();