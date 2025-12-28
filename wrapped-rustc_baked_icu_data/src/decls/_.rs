macro_rules! deps {
    () => {
        BakedDataProvider!();
    };
}

macro_rules! _ {
    () => {
        deps!();
        const _ : () = { impl_data_provider ! (BakedDataProvider) ; } ;
    };
}

_!()