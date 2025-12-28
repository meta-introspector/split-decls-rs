macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! ExtractBundledLibsError {
    () => {
        deps!();
        # [derive (Diagnostic)] pub enum ExtractBundledLibsError < 'a > { # [diag (codegen_ssa_extract_bundled_libs_open_file)] OpenFile { rlib : & 'a Path , error : Box < dyn std :: error :: Error > } , # [diag (codegen_ssa_extract_bundled_libs_mmap_file)] MmapFile { rlib : & 'a Path , error : Box < dyn std :: error :: Error > } , # [diag (codegen_ssa_extract_bundled_libs_parse_archive)] ParseArchive { rlib : & 'a Path , error : Box < dyn std :: error :: Error > } , # [diag (codegen_ssa_extract_bundled_libs_read_entry)] ReadEntry { rlib : & 'a Path , error : Box < dyn std :: error :: Error > } , # [diag (codegen_ssa_extract_bundled_libs_archive_member)] ArchiveMember { rlib : & 'a Path , error : Box < dyn std :: error :: Error > } , # [diag (codegen_ssa_extract_bundled_libs_convert_name)] ConvertName { rlib : & 'a Path , error : Box < dyn std :: error :: Error > } , # [diag (codegen_ssa_extract_bundled_libs_write_file)] WriteFile { rlib : & 'a Path , error : Box < dyn std :: error :: Error > } , # [diag (codegen_ssa_extract_bundled_libs_write_file)] ExtractSection { rlib : & 'a Path , error : Box < dyn std :: error :: Error > } , }
    };
}

ExtractBundledLibsError!()