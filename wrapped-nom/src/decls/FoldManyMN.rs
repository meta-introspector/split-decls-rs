macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! FoldManyMN {
    () => {
        deps!();
        # [doc = " Parser implementation for the [fold_many_m_n] combinator"] pub struct FoldManyMN < F , G , Init , R > { parser : F , g : G , init : Init , r : PhantomData < R > , min : usize , max : usize , }
    };
}

FoldManyMN!();