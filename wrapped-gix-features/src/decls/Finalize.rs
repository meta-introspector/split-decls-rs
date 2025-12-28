macro_rules! deps {
    () => {
        Reduce!();
    };
}

macro_rules! Finalize {
    () => {
        deps!();
        # [doc = " A trait reflecting the `finalize()` method of [`Reduce`] implementations"] pub trait Finalize { # [doc = " An implementation of [`Reduce`]"] type Reduce : self :: Reduce ; # [doc = " Similar to the [`Reduce::finalize()`] method"] fn finalize (self ,) -> Result < < < Self as Finalize > :: Reduce as self :: Reduce > :: Output , < < Self as Finalize > :: Reduce as self :: Reduce > :: Error > ; }
    };
}

Finalize!()