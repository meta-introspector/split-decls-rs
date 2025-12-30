// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
# [doc = " `Either<L, R>` is a future if both `L` and `R` are futures."] impl < L , R > Future for Either < L , R > where L : Future , R : Future < Output = L :: Output > , { type Output = L :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut core :: task :: Context < '_ > ,) -> core :: task :: Poll < Self :: Output > { for_both ! (self . as_pin_mut () , inner => inner . poll (cx)) } }
};
}
