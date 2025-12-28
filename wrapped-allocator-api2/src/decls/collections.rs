macro_rules! deps {
    () => {
        TryReserveErrorKind!();
        TryReserveError!();
    };
}

macro_rules! collections {
    () => {
        deps!();
        # [cfg (feature = "alloc")] pub mod collections { pub use super :: raw_vec :: { TryReserveError , TryReserveErrorKind } ; }
    };
}

collections!();