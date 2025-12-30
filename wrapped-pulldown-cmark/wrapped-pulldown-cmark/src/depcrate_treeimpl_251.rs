// Generated macro for impl_251 (impl)
macro_rules! Depcrate_treeimpl_251 {
() => {
// Module: crate::tree
// Provides: {"impl_251"}
// Dependencies: {}
impl < T > core :: fmt :: Debug for Tree < T > where T : core :: fmt :: Debug , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { fn debug_tree < T > (tree : & Tree < T > , cur : TreeIndex , indent : usize , f : & mut core :: fmt :: Formatter < '_ > ,) -> core :: fmt :: Result where T : core :: fmt :: Debug , { for _ in 0 .. indent { write ! (f , "  ") ? ; } writeln ! (f , "{:?}" , & tree [cur] . item) ? ; if let Some (child_ix) = tree [cur] . child { debug_tree (tree , child_ix , indent + 1 , f) ? ; } if let Some (next_ix) = tree [cur] . next { debug_tree (tree , next_ix , indent , f) ? ; } Ok (()) } if self . nodes . len () > 1 { let cur = TreeIndex (NonZeroUsize :: new (1) . unwrap ()) ; debug_tree (self , cur , 0 , f) } else { write ! (f , "Empty tree") } } }
};
}
