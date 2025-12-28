macro_rules! deps {
    () => {
        ReadVectoredFuture!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl < R : Unpin + ? Sized > Unpin for ReadVectoredFuture < '_ , R > { }
    };
}

impl_269!();