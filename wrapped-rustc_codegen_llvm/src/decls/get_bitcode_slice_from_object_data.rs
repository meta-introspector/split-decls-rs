macro_rules! deps {
    () => {
        LtoBitcodeFromRlib!();
        LlvmCodegenBackend!();
    };
}

macro_rules! get_bitcode_slice_from_object_data {
    () => {
        deps!();
        fn get_bitcode_slice_from_object_data < 'a > (obj : & 'a [u8] , cgcx : & CodegenContext < LlvmCodegenBackend > ,) -> Result < & 'a [u8] , LtoBitcodeFromRlib > { if obj . starts_with (b"\xDE\xC0\x17\x0B") || obj . starts_with (b"BC\xC0\xDE") { return Ok (obj) ; } let section_name = bitcode_section_name (cgcx) . to_str () . unwrap () . trim_start_matches ("__LLVM,") ; let obj = object :: File :: parse (obj) . map_err (| err | LtoBitcodeFromRlib { err : err . to_string () }) ? ; let section = obj . section_by_name (section_name) . ok_or_else (| | LtoBitcodeFromRlib { err : format ! ("Can't find section {section_name}") }) ? ; section . data () . map_err (| err | LtoBitcodeFromRlib { err : err . to_string () }) }
    };
}

get_bitcode_slice_from_object_data!()