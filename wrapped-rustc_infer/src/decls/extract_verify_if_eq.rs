macro_rules! deps {
    () => {
        VerifyIfEq!();
        MatchAgainstHigherRankedOutlives!();
    };
}

macro_rules! extract_verify_if_eq {
    () => {
        deps!();
        # [doc = " Given a \"verify-if-eq\" type test like:"] # [doc = ""] # [doc = " ```rust,ignore (pseudo-Rust)"] # [doc = " exists<'a...> {"] # [doc = "     verify_if_eq(some_type, bound_region)"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " and the type `test_ty` that the type test is being tested against,"] # [doc = " returns:"] # [doc = ""] # [doc = " * `None` if `some_type` cannot be made equal to `test_ty`,"] # [doc = "   no matter the values of the variables in `exists`."] # [doc = " * `Some(r)` with a suitable bound (typically the value of `bound_region`, modulo"] # [doc = "   any bound existential variables, which will be instantiated) for the"] # [doc = "   type under test."] # [doc = ""] # [doc = " NB: This function uses a simplistic, syntactic version of type equality."] # [doc = " In other words, it may spuriously return `None` even if the type-under-test"] # [doc = " is in fact equal to `some_type`. In practice, though, this is used on types"] # [doc = " that are either projections like `T::Item` or `T` and it works fine, but it"] # [doc = " could have trouble when complex types with higher-ranked binders and the"] # [doc = " like are used. This is a particular challenge since this function is invoked"] # [doc = " very late in inference and hence cannot make use of the normal inference"] # [doc = " machinery."] # [instrument (level = "debug" , skip (tcx))] pub fn extract_verify_if_eq < 'tcx > (tcx : TyCtxt < 'tcx > , verify_if_eq_b : & ty :: Binder < 'tcx , VerifyIfEq < 'tcx > > , test_ty : Ty < 'tcx > ,) -> Option < ty :: Region < 'tcx > > { assert ! (! verify_if_eq_b . has_escaping_bound_vars ()) ; let mut m = MatchAgainstHigherRankedOutlives :: new (tcx) ; let verify_if_eq = verify_if_eq_b . skip_binder () ; m . relate (verify_if_eq . ty , test_ty) . ok () ? ; if let ty :: RegionKind :: ReBound (depth , br) = verify_if_eq . bound . kind () { assert ! (depth == ty :: INNERMOST) ; match m . map . get (& br) { Some (& r) => Some (r) , None => { Some (tcx . lifetimes . re_static) } } } else { Some (verify_if_eq . bound) } }
    };
}

extract_verify_if_eq!()