macro_rules! deps {
    () => {
        StreamFooter!();
        Read!();
        Index!();
        Block!();
        StreamHeader!();
        Error!();
        CheckType!();
        Result!();
    };
}

macro_rules! scan_blocks {
    () => {
        deps!();
        # [doc = " Scan the XZ file to collect information about all blocks."] # [doc = " This reads the index at the end of the file to efficiently locate block boundaries."] # [cfg (feature = "std")] fn scan_blocks < R : Read + Seek > (mut reader : R) -> io :: Result < (R , Vec < Block > , CheckType) > { let stream_header = StreamHeader :: parse (& mut reader) ? ; let check_type = stream_header . check_type ; let header_end_pos = reader . stream_position () ? ; let file_size = reader . seek (SeekFrom :: End (0)) ? ; if file_size < 32 { return Err (error_invalid_data ("File too small to contain a valid XZ stream" ,)) ; } reader . seek (SeekFrom :: End (- 12)) ? ; let stream_footer = StreamFooter :: parse (& mut reader) ? ; let header_flags = [0 , check_type as u8] ; if stream_footer . stream_flags != header_flags { return Err (error_invalid_data ("stream header and footer flags mismatch" ,)) ; } let index_size = (stream_footer . backward_size + 1) * 4 ; let index_start_pos = file_size - 12 - index_size as u64 ; reader . seek (SeekFrom :: Start (index_start_pos)) ? ; let index_indicator = reader . read_u8 () ? ; if index_indicator != 0 { return Err (error_invalid_data ("invalid XZ index indicator")) ; } let index = Index :: parse (& mut reader) ? ; let mut blocks = Vec :: new () ; let mut block_start_pos = header_end_pos ; for record in & index . records { blocks . push (Block { start_pos : block_start_pos , unpadded_size : record . unpadded_size , uncompressed_size : record . uncompressed_size , }) ; let padding_needed = (4 - (record . unpadded_size % 4)) % 4 ; let actual_block_size = record . unpadded_size + padding_needed ; block_start_pos += actual_block_size ; } if blocks . is_empty () { return Err (io :: Error :: new (io :: ErrorKind :: InvalidData , "No valid XZ blocks found" ,)) ; } reader . seek (SeekFrom :: Start (0)) ? ; Ok ((reader , blocks , check_type)) }
    };
}

scan_blocks!()