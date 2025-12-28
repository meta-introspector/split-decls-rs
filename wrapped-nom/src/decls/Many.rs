macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Many {
    () => {
        deps!();
        # [doc = " Parser implementation for the [many] combinator"] pub struct Many < F , R , Collection > { parser : F , range : R , c : PhantomData < Collection > , }
    };
}

Many!()