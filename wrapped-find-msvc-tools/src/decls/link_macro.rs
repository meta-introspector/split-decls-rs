macro_rules! link_macro {
    () => {
        macro_rules ! link_macro { ($ library : literal $ abi : literal $ ($ link_name : literal) ? $ (# [$ doc : meta]) ? fn $ ($ function : tt) *) => (# [link (name = "kernel32")] extern $ abi { $ (# [link_name =$ link_name]) ? pub fn $ ($ function) *; }) }
    };
}

link_macro!();