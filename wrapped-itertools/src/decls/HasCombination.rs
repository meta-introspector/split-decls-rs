macro_rules! HasCombination {
    () => {
        pub trait HasCombination < I > : Sized { type Combination : From < I > + Iterator < Item = Self > ; }
    };
}

HasCombination!();