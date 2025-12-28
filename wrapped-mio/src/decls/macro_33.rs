macro_rules! macro_33 {
    () => {
        cfg_os_poll ! { macro_rules ! debug_detail { ($ type : ident ($ event_type : ty) , $ test : path , $ ($ (# [$ target : meta]) * $ libc : ident :: $ flag : ident) ,+ $ (,) *) => { struct $ type ($ event_type) ; impl fmt :: Debug for $ type { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { let mut written_one = false ; $ ($ (# [$ target]) * # [allow (clippy :: bad_bit_mask)] { if $ test (& self . 0 , &$ libc :: $ flag) { if ! written_one { write ! (f , "{}" , stringify ! ($ flag)) ?; written_one = true ; } else { write ! (f , "|{}" , stringify ! ($ flag)) ?; } } }) + if ! written_one { write ! (f , "(empty)") } else { Ok (()) } } } } ; } }
    };
}

macro_33!();