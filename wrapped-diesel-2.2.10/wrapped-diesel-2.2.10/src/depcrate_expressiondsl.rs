// Generated macro for dsl (module)
macro_rules! Depcrate_expressiondsl {
() => {
// Module: crate::expression
// Provides: {"dsl"}
// Dependencies: {}
# [allow (non_camel_case_types , unreachable_pub)] pub (crate) mod dsl { use crate :: dsl :: SqlTypeOf ; # [doc (inline)] pub use super :: case_when :: case_when ; # [doc (inline)] pub use super :: count :: * ; # [doc (inline)] pub use super :: exists :: exists ; # [doc (inline)] pub use super :: functions :: aggregate_folding :: * ; # [doc (inline)] pub use super :: functions :: aggregate_ordering :: * ; # [doc (inline)] pub use super :: functions :: date_and_time :: * ; # [doc (inline)] pub use super :: helper_types :: { case_when , IntoSql , Otherwise , When } ; # [doc (inline)] pub use super :: not :: not ; # [doc (inline)] pub use super :: sql_literal :: sql ; # [cfg (feature = "postgres_backend")] pub use crate :: pg :: expression :: dsl :: * ; # [doc = " The return type of [`count(expr)`](crate::dsl::count())"] pub type count < Expr > = super :: count :: count < SqlTypeOf < Expr > , Expr > ; # [doc = " The return type of [`count_star()`](crate::dsl::count_star())"] pub type count_star = super :: count :: CountStar ; # [doc = " The return type of [`count_distinct()`](crate::dsl::count_distinct())"] pub type count_distinct < Expr > = super :: count :: CountDistinct < SqlTypeOf < Expr > , Expr > ; # [doc = " The return type of [`date(expr)`](crate::dsl::date())"] pub type date < Expr > = super :: functions :: date_and_time :: date < Expr > ; # [cfg (feature = "mysql_backend")] pub use crate :: mysql :: query_builder :: DuplicatedKeys ; }
};
}
