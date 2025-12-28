macro_rules! deps {
    () => {
        Arbitrary!();
        QuickCheck!();
        Gen!();
    };
}

macro_rules! test {
    () => {
        deps!();
        # [cfg (test)] mod test { use crate :: { Gen , QuickCheck } ; # [test] fn shrinking_regression_issue_126 () { fn thetest (vals : Vec < bool >) -> bool { vals . iter () . filter (| & v | * v) . count () < 2 } let failing_case = QuickCheck :: new () . quicktest (thetest as fn (vals : Vec < bool >) -> bool) . unwrap_err () ; let expected_argument = format ! ("{:?}" , [true , true]) ; assert_eq ! (failing_case . arguments , Some (vec ! [expected_argument])) ; } # [test] fn size_for_small_types_issue_143 () { fn t (_ : i8) -> bool { true } QuickCheck :: new () . set_rng (Gen :: new (129)) . quickcheck (t as fn (i8) -> bool) ; } # [test] fn regression_signed_shrinker_panic () { fn foo_can_shrink (v : i8) -> bool { let _ = crate :: Arbitrary :: shrink (& v) . take (100) . count () ; true } crate :: quickcheck (foo_can_shrink as fn (i8) -> bool) ; } }
    };
}

test!()