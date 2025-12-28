macro_rules! deps {
    () => {
        SerializationSink!();
        PageTag!();
        SerializationSinkBuilder!();
        Addr!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: * ; fn test_roundtrip < W > (chunk_size : usize , chunk_count : usize , write : W) where W : Fn (& SerializationSink , & [u8]) -> Addr , { let sink_builder = SerializationSinkBuilder :: new_in_memory () ; let tags = [PageTag :: Events , PageTag :: StringData , PageTag :: StringIndex] ; let expected_chunk : Vec < u8 > = (0 .. chunk_size) . map (| x | (x % 239) as u8) . collect () ; { let sinks : Vec < SerializationSink > = tags . iter () . map (| & tag | sink_builder . new_sink (tag)) . collect () ; for chunk_index in 0 .. chunk_count { let expected_addr = Addr ((chunk_index * chunk_size) as u64) ; for sink in sinks . iter () { assert_eq ! (write (sink , & expected_chunk [..]) , expected_addr) ; } } } let streams : Vec < Vec < u8 > > = tags . iter () . map (| & tag | sink_builder . 0 . copy_bytes_with_page_tag (tag)) . collect () ; for stream in streams { for chunk in stream . chunks (chunk_size) { assert_eq ! (chunk , expected_chunk) ; } } } fn write_closure (sink : & SerializationSink , bytes : & [u8]) -> Addr { sink . write_atomic (bytes . len () , | dest | dest . copy_from_slice (bytes)) } fn write_slice (sink : & SerializationSink , bytes : & [u8]) -> Addr { sink . write_bytes_atomic (bytes) } macro_rules ! mk_roundtrip_test { ($ name : ident , $ chunk_size : expr , $ chunk_count : expr) => { mod $ name { use super ::*; # [test] fn write_atomic () { test_roundtrip ($ chunk_size , $ chunk_count , write_closure) ; } # [test] fn write_bytes_atomic () { test_roundtrip ($ chunk_size , $ chunk_count , write_slice) ; } } } ; } mk_roundtrip_test ! (small_data , 10 , (90 * MAX_PAGE_SIZE) / 100) ; mk_roundtrip_test ! (huge_data , MAX_PAGE_SIZE * 10 , 5) ; mk_roundtrip_test ! (exactly_max_page_size , MAX_PAGE_SIZE , 10) ; mk_roundtrip_test ! (max_page_size_plus_one , MAX_PAGE_SIZE + 1 , 10) ; mk_roundtrip_test ! (max_page_size_minus_one , MAX_PAGE_SIZE - 1 , 10) ; mk_roundtrip_test ! (exactly_min_page_size , MIN_PAGE_SIZE , 10) ; mk_roundtrip_test ! (min_page_size_plus_one , MIN_PAGE_SIZE + 1 , 10) ; mk_roundtrip_test ! (min_page_size_minus_one , MIN_PAGE_SIZE - 1 , 10) ; }
    };
}

tests!();