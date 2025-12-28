macro_rules! deps {
    () => {
        MPlaceTy!();
        Immediate!();
        MemoryKind!();
        CompileTimeInterpCx!();
    };
}

macro_rules! alloc_caller_location {
    () => {
        deps!();
        # [doc = " Allocate a `const core::panic::Location` with the provided filename and line/column numbers."] fn alloc_caller_location < 'tcx > (ecx : & mut CompileTimeInterpCx < 'tcx > , filename : Symbol , line : u32 , col : u32 ,) -> MPlaceTy < 'tcx > { assert ! (! filename . as_str () . as_bytes () . contains (& 0)) ; let loc_details = ecx . tcx . sess . opts . unstable_opts . location_detail ; let filename = { let filename = if loc_details . file { filename . as_str () } else { "<redacted>" } ; let filename_with_nul = filename . to_owned () + "\0" ; let file_ptr = ecx . allocate_bytes_dedup (filename_with_nul . as_bytes ()) . unwrap () ; let file_len = u64 :: try_from (filename . len ()) . unwrap () ; Immediate :: new_slice (file_ptr . into () , file_len , ecx) } ; let line = if loc_details . line { Scalar :: from_u32 (line) } else { Scalar :: from_u32 (0) } ; let col = if loc_details . column { Scalar :: from_u32 (col) } else { Scalar :: from_u32 (0) } ; let loc_ty = ecx . tcx . type_of (ecx . tcx . require_lang_item (LangItem :: PanicLocation , ecx . tcx . span)) . instantiate (* ecx . tcx , ecx . tcx . mk_args (& [ecx . tcx . lifetimes . re_erased . into ()])) ; let loc_layout = ecx . layout_of (loc_ty) . unwrap () ; let location = ecx . allocate (loc_layout , MemoryKind :: CallerLocation) . unwrap () ; let [filename_field , line_field , col_field] = ecx . project_fields (& location , [0 , 1 , 2] . map (FieldIdx :: from_u32)) . unwrap () ; ecx . write_immediate (filename , & filename_field) . expect ("writing to memory we just allocated cannot fail") ; ecx . write_scalar (line , & line_field) . expect ("writing to memory we just allocated cannot fail") ; ecx . write_scalar (col , & col_field) . expect ("writing to memory we just allocated cannot fail") ; location }
    };
}

alloc_caller_location!();