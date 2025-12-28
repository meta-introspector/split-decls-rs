macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! assert_roundtrip {
    () => {
        deps!();
        # [cfg (feature = "proc-macro2")] # [track_caller] pub (crate) fn assert_roundtrip < T > (ours : T , input : & str) where T : std :: convert :: TryFrom < proc_macro2 :: Literal > + fmt :: Debug + PartialEq + Clone , proc_macro2 :: Literal : From < T > , < T as std :: convert :: TryFrom < proc_macro2 :: Literal > > :: Error : std :: fmt :: Display , { let pm_lit = input . parse :: < proc_macro2 :: Literal > () . expect ("failed to parse input as proc_macro2::Literal") ; let t_name = std :: any :: type_name :: < T > () ; if proc_macro2 :: Literal :: from (ours . clone ()) . to_string () != pm_lit . to_string () { panic ! ("Converting {} to proc_macro2::Literal has unexpected result:\
                \nconverted: {:?}\nexpected:  {:?}" , t_name , proc_macro2 :: Literal :: from (ours) , pm_lit ,) ; } match T :: try_from (pm_lit) { Err (e) => { panic ! ("Trying to convert proc_macro2::Literal to {t_name} results in error: {e}") ; } Ok (res) => { if res != ours { panic ! ("Converting proc_macro2::Literal to {t_name} has unexpected result:\n\
                    actual:    {res:?}\n\
                    expected:  {ours:?}") ; } } } }
    };
}

assert_roundtrip!();