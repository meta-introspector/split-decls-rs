macro_rules! deps {
    () => {
        Name!();
        Result!();
    };
}

macro_rules! parse_sysv_extended_name {
    () => {
        deps!();
        # [doc = " Digits are a decimal offset into the extended name table."] # [doc = " Name is terminated by \"/\\n\" (for GNU) or a null byte (for COFF)."] fn parse_sysv_extended_name < 'data > (digits : & [u8] , names : & 'data [u8]) -> Result < & 'data [u8] , () > { let offset = parse_u64_digits (digits , 10) . ok_or (()) ? ; let offset = offset . try_into () . map_err (| _ | ()) ? ; let name_data = names . get (offset ..) . ok_or (()) ? ; let len = memchr :: memchr2 (b'\n' , b'\0' , name_data) . ok_or (()) ? ; if name_data [len] == b'\n' { if len < 1 || name_data [len - 1] != b'/' { Err (()) } else { Ok (& name_data [.. len - 1]) } } else { Ok (& name_data [.. len]) } }
    };
}

parse_sysv_extended_name!()