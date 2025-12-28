macro_rules! deps {
    () => {
        FromParallelIterator!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_1126 {
    () => {
        deps!();
        # [doc = " Collect an arbitrary `Result`-wrapped collection."] # [doc = ""] # [doc = " If any item is `Err`, then all previous `Ok` items collected are"] # [doc = " discarded, and it returns that error.  If there are multiple errors, the"] # [doc = " one returned is not deterministic."] impl < C , T , E > FromParallelIterator < Result < T , E > > for Result < C , E > where C : FromParallelIterator < T > , T : Send , E : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = Result < T , E > > , { fn ok < T , E > (saved : & Mutex < Option < E > >) -> impl Fn (Result < T , E >) -> Option < T > + '_ { move | item | match item { Ok (item) => Some (item) , Err (error) => { if let Ok (mut guard) = saved . try_lock () { if guard . is_none () { * guard = Some (error) ; } } None } } } let saved_error = Mutex :: new (None) ; let collection = par_iter . into_par_iter () . map (ok (& saved_error)) . while_some () . collect () ; match saved_error . into_inner () . unwrap () { Some (error) => Err (error) , None => Ok (collection) , } } }
    };
}

impl_1126!()