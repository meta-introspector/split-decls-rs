// Generated macro for HasTable (trait)
macro_rules! Depcrate_associationsHasTable {
() => {
// Module: crate::associations
// Provides: {"HasTable"}
// Dependencies: {}
# [doc = " This trait indicates that a struct is associated with a single database table."] # [doc = ""] # [doc = " This trait is implemented by structs which implement `Identifiable`,"] # [doc = " as well as database tables themselves."] pub trait HasTable { # [doc = " The table this type is associated with."] type Table : Table ; # [doc = " Returns the table this type is associated with."] fn table () -> Self :: Table ; }
};
}
