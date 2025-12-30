// Generated macro for OwnedExecutor (struct)
macro_rules! Depcrate_executor_owned_executorOwnedExecutor {
() => {
// Module: crate::executor::owned_executor
// Provides: {"OwnedExecutor"}
// Dependencies: {}
# [doc = " [`Executor`] owning all its variables. Can be used after [`Executor`] was"] # [doc = " destroyed."] pub struct OwnedExecutor < 'a , CtxT , S > { pub (super) fragments : HashMap < & 'a str , Fragment < 'a , S > > , pub (super) variables : Variables < S > , pub (super) current_selection_set : Option < Vec < Selection < 'a , S > > > , pub (super) parent_selection_set : Option < Vec < Selection < 'a , S > > > , pub (super) current_type : TypeType < 'a , S > , pub (super) schema : & 'a SchemaType < S > , pub (super) context : & 'a CtxT , pub (super) errors : RwLock < Vec < ExecutionError < S > > > , pub (super) field_path : Arc < FieldPath < 'a > > , }
};
}
