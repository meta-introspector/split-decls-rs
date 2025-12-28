macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_495 {
    () => {
        deps!();
        impl core :: fmt :: Debug for State { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let mut spacing = " " ; for (i , chunk) in self . chunks () . enumerate () { if i > 0 { write ! (f , "{spacing}MATCH") ? ; } spacing = "" ; for (j , t) in chunk . iter () . enumerate () { spacing = " " ; if j == 0 && i > 0 { write ! (f , " ") ? ; } else if j > 0 { write ! (f , ", ") ? ; } write ! (f , "{t:?}") ? ; } } Ok (()) } }
    };
}

impl_495!();