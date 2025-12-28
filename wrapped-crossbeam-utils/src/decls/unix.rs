macro_rules! deps {
    () => {
        ScopedJoinHandle!();
    };
}

macro_rules! unix {
    () => {
        deps!();
        # [doc = " Unix-specific extensions."] # [cfg (unix)] pub mod unix { use super :: ScopedJoinHandle ; use std :: os :: unix :: thread :: JoinHandleExt as _ ; # [doc (no_inline)] pub use std :: os :: unix :: thread :: RawPthread ; mod sealed { pub trait Sealed { } } # [doc = " Unix-specific extensions to [`ScopedJoinHandle`]."] pub trait JoinHandleExt : sealed :: Sealed { # [doc = " Extracts the raw pthread_t without taking ownership"] fn as_pthread_t (& self) -> RawPthread ; # [doc = " Consumes the thread, returning the raw pthread_t"] # [doc = ""] # [doc = " This function **transfers ownership** of the underlying pthread_t to"] # [doc = " the caller. Callers are then the unique owners of the pthread_t and"] # [doc = " must either detach or join the pthread_t once it's no longer needed."] fn into_pthread_t (self) -> RawPthread ; } impl < T > sealed :: Sealed for ScopedJoinHandle < '_ , T > { } impl < T > JoinHandleExt for ScopedJoinHandle < '_ , T > { fn as_pthread_t (& self) -> RawPthread { let handle = self . handle . lock () . unwrap () ; handle . as_ref () . unwrap () . as_pthread_t () } fn into_pthread_t (self) -> RawPthread { self . as_pthread_t () } } }
    };
}

unix!();