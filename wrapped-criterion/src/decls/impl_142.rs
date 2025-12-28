macro_rules! deps {
    () => {
        ReportLink!();
        BenchmarkGroup!();
        BenchmarkId!();
        BenchmarkValueGroup!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < 'a > BenchmarkGroup < 'a > { fn new (output_directory : & Path , ids : & [& 'a BenchmarkId]) -> BenchmarkGroup < 'a > { let group_id = & ids [0] . group_id ; let group_report = ReportLink :: group (output_directory , group_id) ; let mut function_ids = Vec :: with_capacity (ids . len ()) ; let mut values = Vec :: with_capacity (ids . len ()) ; let mut individual_links = HashMap :: with_capacity (ids . len ()) ; for id in ids . iter () { let function_id = id . function_id . as_deref () ; let value = id . value_str . as_deref () ; let individual_link = ReportLink :: individual (output_directory , id) ; function_ids . push (function_id) ; values . push (value) ; individual_links . insert ((function_id , value) , individual_link) ; } fn parse_opt (os : & Option < & str >) -> Option < f64 > { os . and_then (| s | s . parse :: < f64 > () . ok ()) } if values . iter () . all (| os | parse_opt (os) . is_some ()) { values . sort_unstable_by (| v1 , v2 | { let num1 = parse_opt (v1) ; let num2 = parse_opt (v2) ; num1 . partial_cmp (& num2) . unwrap_or (Ordering :: Less) }) ; values . dedup_by_key (| os | parse_opt (os) . unwrap ()) ; } else { values . sort_unstable () ; values . dedup () ; } function_ids . sort_unstable () ; function_ids . dedup () ; let mut value_groups = Vec :: with_capacity (values . len ()) ; for value in values . iter () { let row = function_ids . iter () . filter_map (| f | individual_links . remove (& (* f , * value))) . collect :: < Vec < _ > > () ; value_groups . push (BenchmarkValueGroup { value : value . map (| s | ReportLink :: value (output_directory , group_id , s)) , benchmarks : row , }) ; } let function_ids = function_ids . into_iter () . map (| os | os . map (| s | ReportLink :: function (output_directory , group_id , s))) . collect :: < Option < Vec < _ > > > () ; let values = values . into_iter () . map (| os | os . map (| s | ReportLink :: value (output_directory , group_id , s))) . collect :: < Option < Vec < _ > > > () ; BenchmarkGroup { group_report , function_ids , values , individual_links : value_groups , } } }
    };
}

impl_142!()