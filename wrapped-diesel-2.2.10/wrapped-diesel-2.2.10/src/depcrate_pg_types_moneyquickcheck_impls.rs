// Generated macro for quickcheck_impls (module)
macro_rules! Depcrate_pg_types_moneyquickcheck_impls {
() => {
// Module: crate::pg::types::money
// Provides: {"quickcheck_impls"}
// Dependencies: {}
# [cfg (feature = "quickcheck")] mod quickcheck_impls { extern crate quickcheck ; use self :: quickcheck :: { Arbitrary , Gen } ; use super :: PgMoney ; impl Arbitrary for PgMoney { fn arbitrary (g : & mut Gen) -> Self { PgMoney (i64 :: arbitrary (g)) } } }
};
}
