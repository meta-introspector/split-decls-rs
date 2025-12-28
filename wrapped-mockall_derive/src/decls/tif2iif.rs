macro_rules! tif2iif {
    () => {
        # [doc = " Converts a TraitItemFn into an ImplItemFn"] fn tif2iif (m : syn :: TraitItemFn , vis : & syn :: Visibility) -> syn :: ImplItemFn { let empty_block = Block { brace_token : token :: Brace :: default () , stmts : Vec :: new () } ; syn :: ImplItemFn { attrs : m . attrs , vis : vis . clone () , defaultness : None , sig : m . sig , block : empty_block } }
    };
}

tif2iif!()