macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl State { fn new () -> Self { const IV : [u8 ; 64] = [0x6a , 0x09 , 0xe6 , 0x67 , 0xf3 , 0xbc , 0xc9 , 0x08 , 0xbb , 0x67 , 0xae , 0x85 , 0x84 , 0xca , 0xa7 , 0x3b , 0x3c , 0x6e , 0xf3 , 0x72 , 0xfe , 0x94 , 0xf8 , 0x2b , 0xa5 , 0x4f , 0xf5 , 0x3a , 0x5f , 0x1d , 0x36 , 0xf1 , 0x51 , 0x0e , 0x52 , 0x7f , 0xad , 0xe6 , 0x82 , 0xd1 , 0x9b , 0x05 , 0x68 , 0x8c , 0x2b , 0x3e , 0x6c , 0x1f , 0x1f , 0x83 , 0xd9 , 0xab , 0xfb , 0x41 , 0xbd , 0x6b , 0x5b , 0xe0 , 0xcd , 0x19 , 0x13 , 0x7e , 0x21 , 0x79 ,] ; let mut t = [0u64 ; 8] ; for (i , e) in t . iter_mut () . enumerate () { * e = load_be (& IV , i * 8) } State (t) } # [cfg_attr (feature = "opt_size" , inline (never))] # [cfg_attr (not (feature = "opt_size") , inline (always))] fn add (& mut self , x : & State) { let sx = & mut self . 0 ; let ex = & x . 0 ; sx [0] = sx [0] . wrapping_add (ex [0]) ; sx [1] = sx [1] . wrapping_add (ex [1]) ; sx [2] = sx [2] . wrapping_add (ex [2]) ; sx [3] = sx [3] . wrapping_add (ex [3]) ; sx [4] = sx [4] . wrapping_add (ex [4]) ; sx [5] = sx [5] . wrapping_add (ex [5]) ; sx [6] = sx [6] . wrapping_add (ex [6]) ; sx [7] = sx [7] . wrapping_add (ex [7]) ; } fn store (& self , out : & mut [u8]) { for (i , & e) in self . 0 . iter () . enumerate () { store_be (out , i * 8 , e) ; } } fn blocks (& mut self , mut input : & [u8]) -> usize { let mut t = * self ; let mut inlen = input . len () ; while inlen >= 128 { let mut w = W :: new (input) ; w . G (& mut t , 0) ; w . expand () ; w . G (& mut t , 1) ; w . expand () ; w . G (& mut t , 2) ; w . expand () ; w . G (& mut t , 3) ; w . expand () ; w . G (& mut t , 4) ; t . add (self) ; self . 0 = t . 0 ; input = & input [128 ..] ; inlen -= 128 ; } inlen } }
    };
}

impl_52!();