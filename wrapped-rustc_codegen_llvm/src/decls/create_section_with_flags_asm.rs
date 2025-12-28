macro_rules! create_section_with_flags_asm {
    () => {
        fn create_section_with_flags_asm (section_name : & str , section_flags : & str , data : & [u8]) -> Vec < u8 > { let mut asm = format ! (".section {section_name},\"{section_flags}\"\n") . into_bytes () ; asm . extend_from_slice (b".ascii \"") ; asm . reserve (data . len ()) ; for & byte in data { if byte == b'\\' || byte == b'"' { asm . push (b'\\') ; asm . push (byte) ; } else if byte < 0x20 || byte >= 0x80 { asm . push (b'\\') ; asm . push (b'0' + ((byte >> 6) & 0x7)) ; asm . push (b'0' + ((byte >> 3) & 0x7)) ; asm . push (b'0' + ((byte >> 0) & 0x7)) ; } else { asm . push (byte) ; } } asm . extend_from_slice (b"\"\n") ; asm }
    };
}

create_section_with_flags_asm!()