macro_rules! deps {
    () => {
        Arbitrary!();
        Result!();
        Unstructured!();
    };
}

macro_rules! implement_nonzero_int {
    () => {
        deps!();
        macro_rules ! implement_nonzero_int { ($ nonzero : ty , $ int : ty) => { impl <'a > Arbitrary <'a > for $ nonzero { fn arbitrary (u : & mut Unstructured <'a >) -> Result < Self > { match Self :: new (<$ int as Arbitrary <'a >>:: arbitrary (u) ?) { Some (n) => Ok (n) , None => Ok (Self :: new (<$ int >:: MAX) . unwrap ()) , } } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { <$ int as Arbitrary <'a >>:: size_hint (depth) } } } ; }
    };
}

implement_nonzero_int!()