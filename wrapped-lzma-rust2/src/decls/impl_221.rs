macro_rules! deps {
    () => {
        Result!();
        Write!();
        Lzma2WriterMt!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl < W : Write > Write for Lzma2WriterMt < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { if buf . is_empty () { return Ok (0) ; } let mut total_written = 0 ; let mut remaining_buf = buf ; while ! remaining_buf . is_empty () { let chunk_remaining = self . chunk_size . saturating_sub (self . current_work_unit . len ()) ; let to_write = remaining_buf . len () . min (chunk_remaining) ; if to_write > 0 { self . current_work_unit . extend_from_slice (& remaining_buf [.. to_write]) ; total_written += to_write ; remaining_buf = & remaining_buf [to_write ..] ; } if self . current_work_unit . len () >= self . chunk_size { self . send_work_unit () ? ; } self . drain_available_results () ? ; } Ok (total_written) } fn flush (& mut self) -> io :: Result < () > { if ! self . current_work_unit . is_empty () { self . send_work_unit () ? ; } while let Some (compressed_data) = self . work_pool . try_get_result () ? { self . inner . write_all (& compressed_data) ? ; } self . inner . flush () } }
    };
}

impl_221!();