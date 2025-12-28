macro_rules! deps {
    () => {
        RangeTrie!();
    };
}

macro_rules! impl_571 {
    () => {
        deps!();
        impl fmt :: Debug for RangeTrie { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f) ? ; for (i , state) in self . states . iter () . enumerate () { let status = if i == FINAL . as_usize () { '*' } else { ' ' } ; writeln ! (f , "{status}{i:06}: {state:?}") ? ; } Ok (()) } }
    };
}

impl_571!();