// Generated macro for ObjectFactory (struct)
macro_rules! Depcrate_coff_import_fileObjectFactory {
() => {
// Module: crate::coff_import_file
// Provides: {"ObjectFactory"}
// Dependencies: {}
# [doc = " This class constructs various small object files necessary to support linking"] # [doc = " symbols imported from a DLL.  The contents are pretty strictly defined and"] # [doc = " nearly entirely static.  The details of the structures files are defined in"] # [doc = " WINNT.h and the PE/COFF specification."] struct ObjectFactory < 'a > { native_machine : MachineTypes , import_name : & 'a str , import_descriptor_symbol_name : Vec < u8 > , null_thunk_symbol_name : Vec < u8 > , null_import_descriptor_symbol_name : Vec < u8 > , }
};
}
