macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! FoldMany0 {
    () => {
        deps!();
        # [doc = " Parser implementation for the [fold_many0] combinator"] pub struct FoldMany0 < F , G , Init , R > { parser : F , g : G , init : Init , r : PhantomData < R > , }
    };
}

FoldMany0!()