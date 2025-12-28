macro_rules! HasSource {
    () => {
        pub trait HasSource { type Ast ; # [doc = " Fetches the definition's source node."] # [doc = " Using [`crate::Semantics::source`] is preferred when working with [`crate::Semantics`],"] # [doc = " as that caches the parsed file in the semantics' cache."] # [doc = ""] # [doc = " The current some implementations can return `InFile` instead of `Option<InFile>`."] # [doc = " But we made this method `Option` to support rlib in the future"] # [doc = " by <https://github.com/rust-lang/rust-analyzer/issues/6913>"] fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > ; }
    };
}

HasSource!();