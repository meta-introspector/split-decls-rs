macro_rules! deps {
    () => {
        HuffmanOxide!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl Default for HuffmanOxide { fn default () -> Self { HuffmanOxide { count : [[0 ; MAX_HUFF_SYMBOLS] ; MAX_HUFF_TABLES] , codes : [[0 ; MAX_HUFF_SYMBOLS] ; MAX_HUFF_TABLES] , code_sizes : [[0 ; MAX_HUFF_SYMBOLS] ; MAX_HUFF_TABLES] , } } }
    };
}

impl_72!()