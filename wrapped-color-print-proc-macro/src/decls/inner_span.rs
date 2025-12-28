macro_rules! inner_span {
    () => {
        # [doc = " Unfortunately, unless a nightly compiler is used, this function will actually only return the"] # [doc = " original input span."] # [doc = ""] # [doc = " Returns the subspan corresponding to the range of `inside` inside `input`, considering that:"] # [doc = "  - `input` is exactly `&input_lit_str.value()`,"] # [doc = "  - `inside` is a subslice of `input`,"] # [doc = ""] # [doc = " Warning: may panic if the conditions are not met."] # [doc = " TODO: improve safety"] pub fn inner_span < 'a > (input : & 'a str , input_lit_str : & LitStr , inside : & 'a str) -> Span { let input_offset = (inside . as_ptr () as usize) - (input . as_ptr () as usize) ; let range = input_offset + 1 .. input_offset + inside . len () + 1 ; subspan (input_lit_str . span () , range) . unwrap_or_else (| | input_lit_str . span ()) }
    };
}

inner_span!()