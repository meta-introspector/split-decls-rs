macro_rules! deps {
    () => {
        SampleUniform!();
    };
}

macro_rules! SampleBorrow {
    () => {
        deps!();
        # [doc = " Helper trait similar to [`Borrow`] but implemented"] # [doc = " only for [`SampleUniform`] and references to [`SampleUniform`]"] # [doc = " in order to resolve ambiguity issues."] # [doc = ""] # [doc = " [`Borrow`]: std::borrow::Borrow"] pub trait SampleBorrow < Borrowed > { # [doc = " Immutably borrows from an owned value. See [`Borrow::borrow`]"] # [doc = ""] # [doc = " [`Borrow::borrow`]: std::borrow::Borrow::borrow"] fn borrow (& self) -> & Borrowed ; }
    };
}

SampleBorrow!();