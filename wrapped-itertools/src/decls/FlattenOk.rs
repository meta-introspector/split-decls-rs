macro_rules! FlattenOk {
    () => {
        # [doc = " An iterator adaptor that flattens `Result::Ok` values and"] # [doc = " allows `Result::Err` values through unchanged."] # [doc = ""] # [doc = " See [`.flatten_ok()`](crate::Itertools::flatten_ok) for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct FlattenOk < I , T , E > where I : Iterator < Item = Result < T , E > > , T : IntoIterator , { iter : I , inner_front : Option < T :: IntoIter > , inner_back : Option < T :: IntoIter > , }
    };
}

FlattenOk!();