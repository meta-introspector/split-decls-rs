// Generated macro for anon_object_class_id (function)
macro_rules! Depcrate_read_coff_fileanon_object_class_id {
() => {
// Module: crate::read::coff::file
// Provides: {"anon_object_class_id"}
// Dependencies: {}
# [doc = " Read the `class_id` field from a [`pe::AnonObjectHeader`]."] # [doc = ""] # [doc = " This can be used to determine the format of the header."] pub fn anon_object_class_id < 'data , R : ReadRef < 'data > > (data : R) -> Result < pe :: ClsId > { let header = data . read_at :: < pe :: AnonObjectHeader > (0) . read_error ("Invalid anon object header size or alignment") ? ; Ok (header . class_id) }
};
}
