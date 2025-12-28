macro_rules! deps {
    () => {
        PatternEpsilons!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl core :: fmt :: Debug for PatternEpsilons { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { if self . is_empty () { return write ! (f , "N/A") ; } if let Some (pid) = self . pattern_id () { write ! (f , "{}" , pid . as_usize ()) ? ; } if ! self . epsilons () . is_empty () { if self . pattern_id () . is_some () { write ! (f , "/") ? ; } write ! (f , "{:?}" , self . epsilons ()) ? ; } Ok (()) } }
    };
}

impl_85!()