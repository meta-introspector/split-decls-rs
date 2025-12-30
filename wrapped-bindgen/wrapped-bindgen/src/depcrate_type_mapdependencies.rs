// Generated macro for Dependencies (trait)
macro_rules! Depcrate_type_mapDependencies {
() => {
// Module: crate::type_map
// Provides: {"Dependencies"}
// Dependencies: {}
pub trait Dependencies { fn combine (& self , dependencies : & mut TypeMap) ; fn dependencies (& self) -> TypeMap { let mut dependencies = TypeMap :: new () ; self . combine (& mut dependencies) ; dependencies } }
};
}
