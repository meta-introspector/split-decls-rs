macro_rules! deps {
    () => {
        ImageDosHeader!();
        ImageNtHeaders32!();
        Result!();
        Error!();
        ReadRef!();
    };
}

macro_rules! optional_header_magic {
    () => {
        deps!();
        # [doc = " Find the optional header and read its `magic` field."] # [doc = ""] # [doc = " It can be useful to know this magic value before trying to"] # [doc = " fully parse the NT headers."] pub fn optional_header_magic < 'data , R : ReadRef < 'data > > (data : R) -> Result < u16 > { let dos_header = pe :: ImageDosHeader :: parse (data) ? ; let offset = dos_header . nt_headers_offset () . into () ; let nt_headers = data . read_at :: < pe :: ImageNtHeaders32 > (offset) . read_error ("Invalid NT headers offset, size, or alignment") ? ; if nt_headers . signature () != pe :: IMAGE_NT_SIGNATURE { return Err (Error ("Invalid PE magic")) ; } Ok (nt_headers . optional_header () . magic ()) }
    };
}

optional_header_magic!();