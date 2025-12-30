// Generated macro for PgMoney (struct)
macro_rules! Depcrate_pg_types_moneyPgMoney {
() => {
// Module: crate::pg::types::money
// Provides: {"PgMoney"}
// Dependencies: {}
# [doc = " Money is represented in Postgres as a 64 bit signed integer.  This struct is a dumb wrapper"] # [doc = " type, meant only to indicate the integer's meaning.  The fractional precision of the value is"] # [doc = " determined by the [`lc_monetary` setting of the database](https://www.postgresql.org/docs/9.6/static/datatype-money.html)."] # [doc = " This struct is re-exported as `Cents` as a convenient and conventional expression of a typical"] # [doc = " unit of 1/100th of currency. For other names or precisions, users might consider a differently"] # [doc = " named `use` of the `PgMoney` struct."] # [doc = ""] # [doc = " ```rust"] # [doc = " use diesel::data_types::PgMoney as Pence; // 1/100th unit of Pound"] # [doc = " use diesel::data_types::PgMoney as Fils;  // 1/1000th unit of Dinar"] # [doc = " ```"] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , AsExpression , FromSqlRow)] # [diesel (sql_type = Money)] pub struct PgMoney (pub i64) ;
};
}
