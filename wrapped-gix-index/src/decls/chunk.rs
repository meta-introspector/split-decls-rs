macro_rules! deps {
    () => {
        Entry!();
        Error!();
        Version!();
        Outcome!();
    };
}

macro_rules! chunk {
    () => {
        deps!();
        # [doc = " Note that `data` must point to the beginning of the entries, right past the header."] pub fn chunk < 'a > (mut data : & 'a [u8] , entries : & mut Vec < Entry > , path_backing : & mut Vec < u8 > , num_entries : u32 , object_hash : gix_hash :: Kind , version : Version ,) -> Result < (Outcome , & 'a [u8]) , decode :: Error > { let mut is_sparse = false ; let has_delta_paths = version == Version :: V4 ; let mut prev_path = None ; let mut delta_buf = Vec :: < u8 > :: with_capacity (AVERAGE_V4_DELTA_PATH_LEN_IN_BYTES) ; for idx in 0 .. num_entries { let (entry , remaining) = load_one (data , path_backing , object_hash . len_in_bytes () , has_delta_paths , prev_path ,) . ok_or (decode :: Error :: Entry { index : idx }) ? ; data = remaining ; is_sparse |= entry . mode . is_sparse () ; entries . push (entry) ; prev_path = entries . last () . map (| e | (e . path . clone () , & mut delta_buf)) ; } Ok ((Outcome { is_sparse } , data)) }
    };
}

chunk!()