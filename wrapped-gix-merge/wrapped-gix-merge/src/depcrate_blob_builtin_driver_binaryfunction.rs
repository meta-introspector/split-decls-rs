// Generated macro for function (module)
macro_rules! Depcrate_blob_builtin_driver_binaryfunction {
() => {
// Module: crate::blob::builtin_driver::binary
// Provides: {"function"}
// Dependencies: {}
pub (super) mod function { use crate :: blob :: { builtin_driver :: binary :: { Pick , ResolveWith } , Resolution , } ; # [doc = " As this algorithm doesn't look at the actual data, it returns a choice solely based on logic."] # [doc = " This also means that the caller has to assure this only gets called if the input *doesn't* match."] # [doc = ""] # [doc = " It always results in a conflict with `current` being picked unless `on_conflict` is not `None`,"] # [doc = " which is when we always return [`Resolution::CompleteWithAutoResolvedConflict`]."] pub fn merge (on_conflict : Option < ResolveWith >) -> (Pick , Resolution) { match on_conflict { None => (Pick :: Ours , Resolution :: Conflict) , Some (resolve) => (match resolve { ResolveWith :: Ours => Pick :: Ours , ResolveWith :: Theirs => Pick :: Theirs , ResolveWith :: Ancestor => Pick :: Ancestor , } , Resolution :: CompleteWithAutoResolvedConflict ,) , } } }
};
}
