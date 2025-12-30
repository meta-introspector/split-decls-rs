// Generated macro for impl_2367 (impl)
macro_rules! Depcrate_mysql_connectionimpl_2367 {
() => {
// Module: crate::mysql::connection
// Provides: {"impl_2367"}
// Dependencies: {}
impl MysqlConnection { fn set_config_options (& mut self) -> QueryResult < () > { crate :: sql_query ("SET time_zone = '+00:00';") . execute (self) ? ; crate :: sql_query ("SET character_set_client = 'utf8mb4'") . execute (self) ? ; crate :: sql_query ("SET character_set_connection = 'utf8mb4'") . execute (self) ? ; crate :: sql_query ("SET character_set_results = 'utf8mb4'") . execute (self) ? ; Ok (()) } fn establish_inner (database_url : & str) -> Result < MysqlConnection , ConnectionError > { use crate :: ConnectionError :: CouldntSetupConfiguration ; let raw_connection = RawConnection :: new () ; let connection_options = ConnectionOptions :: parse (database_url) ? ; raw_connection . connect (& connection_options) ? ; let mut conn = MysqlConnection { raw_connection , transaction_state : AnsiTransactionManager :: default () , statement_cache : StatementCache :: new () , instrumentation : None , } ; conn . set_config_options () . map_err (CouldntSetupConfiguration) ? ; Ok (conn) } }
};
}
