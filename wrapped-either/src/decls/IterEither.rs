macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! IterEither {
    () => {
        deps!();
        # [doc = " Iterator that maps left or right iterators to corresponding `Either`-wrapped items."] # [doc = ""] # [doc = " This struct is created by the [`Either::factor_into_iter`],"] # [doc = " [`factor_iter`][Either::factor_iter],"] # [doc = " and [`factor_iter_mut`][Either::factor_iter_mut] methods."] # [derive (Clone , Debug)] pub struct IterEither < L , R > { inner : Either < L , R > , }
    };
}

IterEither!();