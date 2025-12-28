macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! parse_impl {
    () => {
        deps!();
        # [doc = " Precondition: input has to start with either `\"` or `r`."] # [inline (never)] pub (crate) fn parse_impl (input : & str) -> Result < (Option < String > , Option < u8 > , usize) , ParseError > { if input . starts_with ('r') { scan_raw_string (input , 1 , true , true) . map (| (hashes , start_suffix) | (None , Some (hashes) , start_suffix)) } else { unescape_string :: < String > (input , 1 , true , false , true) . map (| (v , start_suffix) | (v , None , start_suffix)) } }
    };
}

parse_impl!()