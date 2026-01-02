mkuse!{use std :: ffi :: CString ;}
mkuse!{use crate :: common :: AsCCharPtr ;}
mkuse!{use crate :: coverageinfo :: ffi ;}
mkuse!{use crate :: llvm ;}

macro_rules! covmap_var_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function covmap_var_name in module {}", module_path!());
    };
}

mkfn!{
    covmap_var_name_introspect!();
    pub (crate) fn covmap_var_name () -> CString { CString :: new (llvm :: build_byte_buffer (| s | unsafe { llvm :: LLVMRustCoverageWriteCovmapVarNameToString (s) ; })) . expect ("covmap variable name should not contain NUL") }
}

macro_rules! covmap_section_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function covmap_section_name in module {}", module_path!());
    };
}

mkfn!{
    covmap_section_name_introspect!();
    pub (crate) fn covmap_section_name (llmod : & llvm :: Module) -> CString { CString :: new (llvm :: build_byte_buffer (| s | unsafe { llvm :: LLVMRustCoverageWriteCovmapSectionNameToString (llmod , s) ; })) . expect ("covmap section name should not contain NUL") }
}

macro_rules! covfun_section_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function covfun_section_name in module {}", module_path!());
    };
}

mkfn!{
    covfun_section_name_introspect!();
    pub (crate) fn covfun_section_name (llmod : & llvm :: Module) -> CString { CString :: new (llvm :: build_byte_buffer (| s | unsafe { llvm :: LLVMRustCoverageWriteCovfunSectionNameToString (llmod , s) ; })) . expect ("covfun section name should not contain NUL") }
}

macro_rules! create_pgo_func_name_var_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_pgo_func_name_var in module {}", module_path!());
    };
}

mkfn!{
    create_pgo_func_name_var_introspect!();
    pub (crate) fn create_pgo_func_name_var < 'll > (llfn : & 'll llvm :: Value , mangled_fn_name : & str ,) -> & 'll llvm :: Value { unsafe { llvm :: LLVMRustCoverageCreatePGOFuncNameVar (llfn , mangled_fn_name . as_c_char_ptr () , mangled_fn_name . len () ,) } }
}

macro_rules! write_filenames_to_buffer_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_filenames_to_buffer in module {}", module_path!());
    };
}

mkfn!{
    write_filenames_to_buffer_introspect!();
    pub (crate) fn write_filenames_to_buffer (filenames : & [impl AsRef < str >]) -> Vec < u8 > { let (pointers , lengths) = filenames . into_iter () . map (AsRef :: as_ref) . map (| s : & str | (s . as_c_char_ptr () , s . len ())) . unzip :: < _ , _ , Vec < _ > , Vec < _ > > () ; llvm :: build_byte_buffer (| buffer | unsafe { llvm :: LLVMRustCoverageWriteFilenamesToBuffer (pointers . as_ptr () , pointers . len () , lengths . as_ptr () , lengths . len () , buffer ,) ; }) }
}

macro_rules! write_function_mappings_to_buffer_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_function_mappings_to_buffer in module {}", module_path!());
    };
}

mkfn!{
    write_function_mappings_to_buffer_introspect!();
    pub (crate) fn write_function_mappings_to_buffer (virtual_file_mapping : & [u32] , expressions : & [ffi :: CounterExpression] , regions : & ffi :: Regions ,) -> Vec < u8 > { let ffi :: Regions { code_regions , expansion_regions , branch_regions } = regions ; llvm :: build_byte_buffer (| buffer | unsafe { llvm :: LLVMRustCoverageWriteFunctionMappingsToBuffer (virtual_file_mapping . as_ptr () , virtual_file_mapping . len () , expressions . as_ptr () , expressions . len () , code_regions . as_ptr () , code_regions . len () , expansion_regions . as_ptr () , expansion_regions . len () , branch_regions . as_ptr () , branch_regions . len () , buffer ,) }) }
}

macro_rules! hash_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hash_bytes in module {}", module_path!());
    };
}

mkfn!{
    hash_bytes_introspect!();
    # [doc = " Hashes some bytes into a 64-bit hash, via LLVM's `IndexedInstrProf::ComputeHash`,"] # [doc = " as required for parts of the LLVM coverage mapping format."] pub (crate) fn hash_bytes (bytes : & [u8]) -> u64 { unsafe { llvm :: LLVMRustCoverageHashBytes (bytes . as_c_char_ptr () , bytes . len ()) } }
}

macro_rules! mapping_version_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mapping_version in module {}", module_path!());
    };
}

mkfn!{
    mapping_version_introspect!();
    # [doc = " Returns LLVM's `coverage::CovMapVersion::CurrentVersion` (CoverageMapping.h)"] # [doc = " as a raw numeric value. For historical reasons, the numeric value is 1 less"] # [doc = " than the number in the version's name, so `Version7` is actually `6u32`."] pub (crate) fn mapping_version () -> u32 { unsafe { llvm :: LLVMRustCoverageMappingVersion () } }
}