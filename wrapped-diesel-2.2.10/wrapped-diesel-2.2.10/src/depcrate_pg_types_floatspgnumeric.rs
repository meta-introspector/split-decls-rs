// Generated macro for PgNumeric (enum)
macro_rules! Depcrate_pg_types_floatsPgNumeric {
() => {
// Module: crate::pg::types::floats
// Provides: {"PgNumeric"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , AsExpression , FromSqlRow)] # [diesel (sql_type = sql_types :: Numeric)] # [doc = " Represents a NUMERIC value, closely mirroring the PG wire protocol"] # [doc = " representation"] pub enum PgNumeric { # [doc = " A positive number"] Positive { # [doc = " How many digits come before the decimal point?"] weight : i16 , # [doc = " How many significant digits are there?"] scale : u16 , # [doc = " The digits in this number, stored in base 10000"] digits : Vec < i16 > , } , # [doc = " A negative number"] Negative { # [doc = " How many digits come before the decimal point?"] weight : i16 , # [doc = " How many significant digits are there?"] scale : u16 , # [doc = " The digits in this number, stored in base 10000"] digits : Vec < i16 > , } , # [doc = " Not a number"] NaN , }
};
}
