macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! FoldMany1 {
    () => {
        deps!();
        # [doc = " Parser implementation for the [fold_many1] combinator"] pub struct FoldMany1 < F , G , Init , R > { parser : F , g : G , init : Init , r : PhantomData < R > , }
    };
}

FoldMany1!()