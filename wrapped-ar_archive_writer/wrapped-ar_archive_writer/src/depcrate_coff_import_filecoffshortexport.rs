// Generated macro for COFFShortExport (struct)
macro_rules! Depcrate_coff_import_fileCOFFShortExport {
() => {
// Module: crate::coff_import_file
// Provides: {"COFFShortExport"}
// Dependencies: {}
pub struct COFFShortExport { # [doc = " The name of the export as specified in the .def file or on the command"] # [doc = " line, i.e. \"foo\" in \"/EXPORT:foo\", and \"bar\" in \"/EXPORT:foo=bar\". This"] # [doc = " may lack mangling, such as underscore prefixing and stdcall suffixing."] pub name : String , # [doc = " The external, exported name. Only non-empty when export renaming is in"] # [doc = " effect, i.e. \"foo\" in \"/EXPORT:foo=bar\"."] pub ext_name : Option < String > , # [doc = " The real, mangled symbol name from the object file. Given"] # [doc = " \"/export:foo=bar\", this could be \"_bar@8\" if bar is stdcall."] pub symbol_name : Option < String > , # [doc = " Creates an import library entry that imports from a DLL export with a"] # [doc = " different name. This is the name of the DLL export that should be"] # [doc = " referenced when linking against this import library entry. In a .def"] # [doc = " file, this is \"baz\" in \"EXPORTS\\nfoo = bar == baz\"."] pub import_name : Option < String > , # [doc = " Specifies EXPORTAS name. In a .def file, this is \"bar\" in"] # [doc = " \"EXPORTS\\nfoo EXPORTAS bar\"."] pub export_as : Option < String > , pub ordinal : u16 , pub noname : bool , pub data : bool , pub private : bool , pub constant : bool , }
};
}
