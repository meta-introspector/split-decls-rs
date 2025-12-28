macro_rules! private_try_future {
    () => {
        mod private_try_future { use super :: Future ; pub trait Sealed { } impl < F , T , E > Sealed for F where F : ? Sized + Future < Output = Result < T , E > > { } }
    };
}

private_try_future!();