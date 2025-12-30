// Generated macro for alloc_support (module)
macro_rules! Depcratealloc_support {
() => {
// Module: crate
// Provides: {"alloc_support"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod alloc_support { use super :: * ; use crate :: std :: boxed :: Box ; impl_value_ref_forward ! ({ impl <'sval , T : ValueRef <'sval > + ? Sized > ValueRef <'sval > for Box < T > } => x => { ** x }) ; }
};
}
