macro_rules! deps {
    () => {
        TransactionOperation!();
        Savepoint!();
        Transaction!();
    };
}

macro_rules! AuthAction {
    () => {
        deps!();
        # [doc = " Actions and arguments found within a statement during"] # [doc = " preparation."] # [doc = ""] # [doc = " See <https://sqlite.org/c3ref/c_alter_table.html> for more info."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] # [non_exhaustive] # [allow (missing_docs)] pub enum AuthAction < 'c > { # [doc = " This variant is not normally produced by SQLite. You may encounter it"] Unknown { # [doc = " The unknown authorization action code."] code : i32 , # [doc = " The third arg to the authorizer callback."] arg1 : Option < & 'c str > , # [doc = " The fourth arg to the authorizer callback."] arg2 : Option < & 'c str > , } , CreateIndex { index_name : & 'c str , table_name : & 'c str , } , CreateTable { table_name : & 'c str , } , CreateTempIndex { index_name : & 'c str , table_name : & 'c str , } , CreateTempTable { table_name : & 'c str , } , CreateTempTrigger { trigger_name : & 'c str , table_name : & 'c str , } , CreateTempView { view_name : & 'c str , } , CreateTrigger { trigger_name : & 'c str , table_name : & 'c str , } , CreateView { view_name : & 'c str , } , Delete { table_name : & 'c str , } , DropIndex { index_name : & 'c str , table_name : & 'c str , } , DropTable { table_name : & 'c str , } , DropTempIndex { index_name : & 'c str , table_name : & 'c str , } , DropTempTable { table_name : & 'c str , } , DropTempTrigger { trigger_name : & 'c str , table_name : & 'c str , } , DropTempView { view_name : & 'c str , } , DropTrigger { trigger_name : & 'c str , table_name : & 'c str , } , DropView { view_name : & 'c str , } , Insert { table_name : & 'c str , } , Pragma { pragma_name : & 'c str , # [doc = " The pragma value, if present (e.g., `PRAGMA name = value;`)."] pragma_value : Option < & 'c str > , } , Read { table_name : & 'c str , column_name : & 'c str , } , Select , Transaction { operation : TransactionOperation , } , Update { table_name : & 'c str , column_name : & 'c str , } , Attach { filename : & 'c str , } , Detach { database_name : & 'c str , } , AlterTable { database_name : & 'c str , table_name : & 'c str , } , Reindex { index_name : & 'c str , } , Analyze { table_name : & 'c str , } , CreateVtable { table_name : & 'c str , module_name : & 'c str , } , DropVtable { table_name : & 'c str , module_name : & 'c str , } , Function { function_name : & 'c str , } , Savepoint { operation : TransactionOperation , savepoint_name : & 'c str , } , Recursive , }
    };
}

AuthAction!()