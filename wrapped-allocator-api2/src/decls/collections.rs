macro_rules! collections {
    () => {
        # [cfg (feature = "alloc")] pub mod collections { pub use super :: raw_vec :: { TryReserveError , TryReserveErrorKind } ; }
    };
}

collections!()