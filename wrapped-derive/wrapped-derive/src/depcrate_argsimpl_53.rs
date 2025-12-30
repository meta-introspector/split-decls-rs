// Generated macro for impl_53 (impl)
macro_rules! Depcrate_argsimpl_53 {
() => {
// Module: crate::args
// Provides: {"impl_53"}
// Dependencies: {}
impl RenameTarget { fn rule (& self) -> RenameRule { match self { RenameTarget :: Type => RenameRule :: Pascal , RenameTarget :: EnumItem => RenameRule :: ScreamingSnake , RenameTarget :: Field => RenameRule :: Camel , RenameTarget :: Argument => RenameRule :: Camel , } } pub fn rename (& self , name : impl AsRef < str >) -> String { self . rule () . rename (name) } }
};
}
