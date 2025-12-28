macro_rules! deps {
    () => {
        HirTyCtxt!();
        AnonConst!();
    };
}

macro_rules! nested_filter {
    () => {
        deps!();
        pub mod nested_filter { use super :: HirTyCtxt ; # [doc = " Specifies what nested things a visitor wants to visit. By \"nested"] # [doc = " things\", we are referring to bits of HIR that are not directly embedded"] # [doc = " within one another but rather indirectly, through a table in the crate."] # [doc = " This is done to control dependencies during incremental compilation: the"] # [doc = " non-inline bits of HIR can be tracked and hashed separately."] # [doc = ""] # [doc = " The most common choice is `OnlyBodies`, which will cause the visitor to"] # [doc = " visit fn bodies for fns that it encounters, and closure bodies, but"] # [doc = " skip over nested item-like things."] # [doc = ""] # [doc = " See the comments at [`rustc_hir::intravisit`] for more details on the overall"] # [doc = " visit strategy."] pub trait NestedFilter < 'hir > { type MaybeTyCtxt : HirTyCtxt < 'hir > ; # [doc = " Whether the visitor visits nested \"item-like\" things."] # [doc = " E.g., item, impl-item."] const INTER : bool ; # [doc = " Whether the visitor visits \"intra item-like\" things."] # [doc = " E.g., function body, closure, `AnonConst`"] const INTRA : bool ; } # [doc = " Do not visit any nested things. When you add a new"] # [doc = " \"non-nested\" thing, you will want to audit such uses to see if"] # [doc = " they remain valid."] # [doc = ""] # [doc = " Use this if you are only walking some particular kind of tree"] # [doc = " (i.e., a type, or fn signature) and you don't want to thread a"] # [doc = " `tcx` around."] pub struct None (()) ; impl NestedFilter < '_ > for None { type MaybeTyCtxt = ! ; const INTER : bool = false ; const INTRA : bool = false ; } }
    };
}

nested_filter!();