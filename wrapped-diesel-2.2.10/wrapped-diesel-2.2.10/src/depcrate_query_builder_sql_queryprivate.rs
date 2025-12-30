// Generated macro for private (module)
macro_rules! Depcrate_query_builder_sql_queryprivate {
() => {
// Module: crate::query_builder::sql_query
// Provides: {"private"}
// Dependencies: {}
mod private { use crate :: backend :: { Backend , DieselReserveSpecialization } ; use crate :: query_builder :: { QueryFragment , QueryId } ; # [derive (Debug , Clone , Copy , QueryId)] pub struct Empty ; impl < DB > QueryFragment < DB > for Empty where DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , _pass : crate :: query_builder :: AstPass < '_ , 'b , DB > ,) -> crate :: QueryResult < () > { Ok (()) } } }
};
}
