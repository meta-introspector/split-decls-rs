// Generated macro for ops (module)
macro_rules! Depcrate_pg_typesops {
() => {
// Module: crate::pg::types
// Provides: {"ops"}
// Dependencies: {}
mod ops { use super :: sql_types :: * ; use crate :: sql_types :: ops :: * ; use crate :: sql_types :: { Bigint , Interval } ; impl Add for Timestamptz { type Rhs = Interval ; type Output = Timestamptz ; } impl Sub for Timestamptz { type Rhs = Interval ; type Output = Timestamptz ; } impl Add for Cidr { type Rhs = Bigint ; type Output = Inet ; } impl Add for Inet { type Rhs = Bigint ; type Output = Inet ; } impl Sub for Cidr { type Rhs = Bigint ; type Output = Inet ; } impl Sub for Inet { type Rhs = Bigint ; type Output = Inet ; } }
};
}
