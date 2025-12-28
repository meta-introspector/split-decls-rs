macro_rules! expect {
    () => {
        # [doc = " Workaround because `.expect()` is not (yet) available in const context."] pub (crate) const fn expect < T : Copy > (opt : Option < T > , msg : & str) -> T { match opt { Some (val) => val , None => panic ! ("{}" , msg) , } }
    };
}

expect!();