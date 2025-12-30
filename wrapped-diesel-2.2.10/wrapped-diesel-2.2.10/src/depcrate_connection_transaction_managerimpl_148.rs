// Generated macro for impl_148 (impl)
macro_rules! Depcrate_connection_transaction_managerimpl_148 {
() => {
// Module: crate::connection::transaction_manager
// Provides: {"impl_148"}
// Dependencies: {}
impl ValidTransactionManagerStatus { # [doc = " Return the current transaction depth"] # [doc = ""] # [doc = " This value is `None` if no current transaction is running"] # [doc = " otherwise the number of nested transactions is returned."] pub fn transaction_depth (& self) -> Option < NonZeroU32 > { self . in_transaction . as_ref () . map (| it | it . transaction_depth) } # [doc = " Update the transaction depth by adding the value of the `transaction_depth_change` parameter if the `query` is"] # [doc = " `Ok(())`"] pub fn change_transaction_depth (& mut self , transaction_depth_change : TransactionDepthChange ,) -> QueryResult < () > { match (& mut self . in_transaction , transaction_depth_change) { (Some (in_transaction) , TransactionDepthChange :: IncreaseDepth) => { in_transaction . transaction_depth = NonZeroU32 :: new (in_transaction . transaction_depth . get () . saturating_add (1)) . expect ("nz + nz is always non-zero") ; Ok (()) } (Some (in_transaction) , TransactionDepthChange :: DecreaseDepth) => { match NonZeroU32 :: new (in_transaction . transaction_depth . get () - 1) { Some (depth) => in_transaction . transaction_depth = depth , None => self . in_transaction = None , } Ok (()) } (None , TransactionDepthChange :: IncreaseDepth) => { self . in_transaction = Some (InTransactionStatus { transaction_depth : NonZeroU32 :: new (1) . expect ("1 is non-zero") , requires_rollback_maybe_up_to_top_level : false , test_transaction : false , }) ; Ok (()) } (None , TransactionDepthChange :: DecreaseDepth) => { Err (Error :: NotInTransaction) } } } }
};
}
