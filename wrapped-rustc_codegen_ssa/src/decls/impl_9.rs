macro_rules! deps {
    () => {
        IncorrectCguReuseType!();
        CguNotRecorded!();
        TrackerData!();
        CguReuse!();
        CguReuseTracker!();
        ComparisonKind!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl CguReuseTracker { fn new () -> CguReuseTracker { let data = TrackerData { actual_reuse : Default :: default () , expected_reuse : Default :: default () } ; CguReuseTracker { data : Some (data) } } fn new_disabled () -> CguReuseTracker { CguReuseTracker { data : None } } pub fn set_actual_reuse (& mut self , cgu_name : & str , kind : CguReuse) { if let Some (data) = & mut self . data { debug ! ("set_actual_reuse({cgu_name:?}, {kind:?})") ; let prev_reuse = data . actual_reuse . insert (cgu_name . to_string () , kind) ; assert ! (prev_reuse . is_none ()) ; } } fn set_expectation (& mut self , cgu_name : Symbol , cgu_user_name : & str , error_span : Span , expected_reuse : CguReuse , comparison_kind : ComparisonKind ,) { if let Some (data) = & mut self . data { debug ! ("set_expectation({cgu_name:?}, {expected_reuse:?}, {comparison_kind:?})") ; data . expected_reuse . insert (cgu_name . to_string () , (cgu_user_name . to_string () , error_span , expected_reuse , comparison_kind) ,) ; } } fn check_expected_reuse (& self , sess : & Session) { if let Some (ref data) = self . data { let keys = data . expected_reuse . keys () . into_sorted_stable_ord () ; for cgu_name in keys { let & (ref cgu_user_name , ref error_span , expected_reuse , comparison_kind) = data . expected_reuse . get (cgu_name) . unwrap () ; if let Some (& actual_reuse) = data . actual_reuse . get (cgu_name) { let (error , at_least) = match comparison_kind { ComparisonKind :: Exact => (expected_reuse != actual_reuse , false) , ComparisonKind :: AtLeast => (actual_reuse < expected_reuse , true) , } ; if error { let at_least = if at_least { 1 } else { 0 } ; sess . dcx () . emit_err (errors :: IncorrectCguReuseType { span : * error_span , cgu_user_name , actual_reuse , expected_reuse , at_least , }) ; } } else { sess . dcx () . emit_fatal (errors :: CguNotRecorded { cgu_user_name , cgu_name }) ; } } } } }
    };
}

impl_9!()