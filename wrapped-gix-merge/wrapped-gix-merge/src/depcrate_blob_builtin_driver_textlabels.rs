// Generated macro for Labels (struct)
macro_rules! Depcrate_blob_builtin_driver_textLabels {
() => {
// Module: crate::blob::builtin_driver::text
// Provides: {"Labels"}
// Dependencies: {}
# [doc = " The set of labels to annotate conflict markers with."] # [doc = ""] # [doc = " That way it becomes clearer where the content of conflicts are originating from."] # [derive (Default , Copy , Clone , Debug , Eq , PartialEq)] pub struct Labels < 'a > { # [doc = " The label for the common *ancestor*."] pub ancestor : Option < & 'a BStr > , # [doc = " The label for the *current* (or *our*) side."] pub current : Option < & 'a BStr > , # [doc = " The label for the *other* (or *their*) side."] pub other : Option < & 'a BStr > , }
};
}
