macro_rules! unique_masks {
    () => {
        # [doc = " Given a list of identifiers, assign each one a unique sequential single-bit mask."] # [allow (unused_macros)] macro_rules ! unique_masks { ($ ty : ty , $ ($ name : ident ,) +) => { # [cfg (test)] pub const ALL : & [$ ty] = & [$ ($ name) ,+] ; # [cfg (test)] pub const NAMES : & [& str] = & [$ (stringify ! ($ name)) ,+] ; unique_masks ! (@ one ; $ ty ; 0 ; $ ($ name ,) +) ; } ; (@ one ; $ _ty : ty ; $ _idx : expr ;) => { } ; (@ one ; $ ty : ty ; $ shift : expr ; $ name : ident , $ ($ tail : tt) *) => { pub const $ name : $ ty = 1 << $ shift ; const _ : () = assert ! ($ name != (1 << (<$ ty >:: BITS - 1))) ; unique_masks ! (@ one ; $ ty ; $ shift + 1 ; $ ($ tail) *) ; } ; }
    };
}

unique_masks!();