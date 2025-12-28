macro_rules! make_trait_method {
    () => {
        fn make_trait_method (sig : syn :: Signature) -> TraitItemFn { TraitItemFn { attrs : vec ! [] , sig : sig . clone () , semi_token : Some (syn :: Token ! [;] (sig . span ())) , default : None , } }
    };
}

make_trait_method!()