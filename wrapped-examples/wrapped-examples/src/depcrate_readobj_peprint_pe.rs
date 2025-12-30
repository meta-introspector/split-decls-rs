// Generated macro for print_pe (function)
macro_rules! Depcrate_readobj_peprint_pe {
() => {
// Module: crate::readobj::pe
// Provides: {"print_pe"}
// Dependencies: {}
fn print_pe < Pe : ImageNtHeaders > (p : & mut Printer < '_ > , data : & [u8]) { if let Some (dos_header) = ImageDosHeader :: parse (data) . print_err (p) { print_dos (p , dos_header) ; let mut offset = dos_header . nt_headers_offset () . into () ; print_rich (p , data , offset) ; if let Some ((nt_headers , data_directories)) = Pe :: parse (data , & mut offset) . print_err (p) { if p . options . file { p . group ("ImageNtHeaders" , | p | { p . field_hex ("Signature" , nt_headers . signature ()) ; }) ; } let header = nt_headers . file_header () ; let machine = header . machine . get (LE) ; let sections = header . sections (data , offset) . print_err (p) ; let symbols = header . symbols (data) . print_err (p) ; print_file (p , header) ; print_optional (p , nt_headers . optional_header ()) ; print_data_directories (p , & data_directories) ; if let Some (ref sections) = sections { print_sections (p , data , machine , symbols . as_ref () , sections) ; } if let Some (ref symbols) = symbols { print_symbols (p , sections . as_ref () , symbols) ; } if let Some (ref sections) = sections { print_export_dir (p , data , sections , & data_directories) ; print_import_dir :: < Pe > (p , data , sections , & data_directories) ; print_delay_load_dir :: < Pe > (p , data , sections , & data_directories) ; print_reloc_dir (p , data , machine , sections , & data_directories) ; print_resource_dir (p , data , sections , & data_directories) ; } } } }
};
}
