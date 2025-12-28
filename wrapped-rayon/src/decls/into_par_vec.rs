macro_rules! deps {
    () => {
        Iter!();
        IntoParallelIterator!();
    };
}

macro_rules! into_par_vec {
    () => {
        deps!();
        # [doc = " Convert an iterable collection into a parallel iterator by first"] # [doc = " collecting into a temporary `Vec`, then iterating that."] macro_rules ! into_par_vec { ($ t : ty => $ iter : ident <$ ($ i : tt) ,*>, impl $ ($ args : tt) *) => { impl $ ($ args) * IntoParallelIterator for $ t { type Item = <$ t as IntoIterator >:: Item ; type Iter = $ iter <$ ($ i) ,*>; fn into_par_iter (self) -> Self :: Iter { use std :: iter :: FromIterator ; $ iter { inner : Vec :: from_iter (self) . into_par_iter () } } } } ; }
    };
}

into_par_vec!()