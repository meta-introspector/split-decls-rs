// Generated macro for MaybeResPath (trait)
macro_rules! Depcrate_resMaybeResPath {
() => {
// Module: crate::res
// Provides: {"MaybeResPath"}
// Dependencies: {}
# [doc = " A HIR node which might be a `QPath::Resolved`."] # [doc = ""] # [doc = " The following are resolved paths:"] # [doc = " * A path to a module or crate item."] # [doc = " * A path to a trait item via the trait's name."] # [doc = " * A path to a struct or variant constructor via the original type's path."] # [doc = " * A local."] # [doc = ""] # [doc = " All other paths are `TypeRelative` and require using `PathRes` to lookup the"] # [doc = " resolution."] pub trait MaybeResPath < 'a > : Copy { # [doc = " If this node is a resolved path gets both the contained path and the"] # [doc = " type associated with it."] fn opt_res_path (self) -> OptResPath < 'a > ; # [doc = " If this node is a resolved path gets it's resolution. Returns `Res::Err`"] # [doc = " otherwise."] # [inline] fn basic_res (self) -> & 'a Res { self . opt_res_path () . 1 . map_or (& Res :: Err , | p | & p . res) } # [doc = " If this node is a path to a local gets the local's `HirId`."] # [inline] fn res_local_id (self) -> Option < HirId > { if let (_ , Some (p)) = self . opt_res_path () && let Res :: Local (id) = p . res { Some (id) } else { None } } # [doc = " If this node is a path to a local gets the local's `HirId` and identifier."] fn res_local_id_and_ident (self) -> Option < (HirId , & 'a Ident) > { if let (_ , Some (p)) = self . opt_res_path () && let Res :: Local (id) = p . res && let [seg] = p . segments { Some ((id , & seg . ident)) } else { None } } }
};
}
