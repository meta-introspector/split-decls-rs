// Generated macro for impl_28 (impl)
macro_rules! Depcrate_eventimpl_28 {
() => {
// Module: crate::event
// Provides: {"impl_28"}
// Dependencies: {}
impl EventKind { # [doc = " Indicates whether an event is an Access variant."] pub fn is_access (& self) -> bool { matches ! (self , EventKind :: Access (_)) } # [doc = " Indicates whether an event is a Create variant."] pub fn is_create (& self) -> bool { matches ! (self , EventKind :: Create (_)) } # [doc = " Indicates whether an event is a Modify variant."] pub fn is_modify (& self) -> bool { matches ! (self , EventKind :: Modify (_)) } # [doc = " Indicates whether an event is a Remove variant."] pub fn is_remove (& self) -> bool { matches ! (self , EventKind :: Remove (_)) } # [doc = " Indicates whether an event is an Other variant."] pub fn is_other (& self) -> bool { matches ! (self , EventKind :: Other) } }
};
}
