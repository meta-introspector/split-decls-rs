macro_rules! HuffmanOxide {
    () => {
        # [doc = " A struct containing data about huffman codes and symbol frequencies."] # [doc = ""] # [doc = " NOTE: Only the literal/lengths have enough symbols to actually use"] # [doc = " the full array. It's unclear why it's defined like this in miniz,"] # [doc = " it could be for cache/alignment reasons."] pub (crate) struct HuffmanOxide { # [doc = " Number of occurrences of each symbol."] pub count : [[u16 ; MAX_HUFF_SYMBOLS] ; MAX_HUFF_TABLES] , # [doc = " The bits of the huffman code assigned to the symbol"] pub codes : [[u16 ; MAX_HUFF_SYMBOLS] ; MAX_HUFF_TABLES] , # [doc = " The length of the huffman code assigned to the symbol."] pub code_sizes : [[u8 ; MAX_HUFF_SYMBOLS] ; MAX_HUFF_TABLES] , }
    };
}

HuffmanOxide!()