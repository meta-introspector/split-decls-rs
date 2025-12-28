macro_rules! deps {
    () => {
        Sample!();
        ValueFormatter!();
        BenchmarkId!();
    };
}

macro_rules! line_comparison_series_data {
    () => {
        deps!();
        # [allow (clippy :: type_complexity)] fn line_comparison_series_data < 'a > (formatter : & dyn ValueFormatter , all_curves : & [& (& 'a BenchmarkId , Vec < f64 >)] ,) -> (& 'static str , Vec < (Option < & 'a String > , Vec < f64 > , Vec < f64 >) >) { let max = all_curves . iter () . map (| & (_ , data) | Sample :: new (data) . mean ()) . fold (f64 :: NAN , f64 :: max) ; let mut dummy = [1.0] ; let unit = formatter . scale_values (max , & mut dummy) ; let mut series_data = vec ! [] ; for (key , group) in & all_curves . iter () . chunk_by (| & & & (id , _) | & id . function_id) { let mut tuples : Vec < _ > = group . map (| & & (id , ref sample) | { let x = id . as_number () . unwrap () ; let y = Sample :: new (sample) . mean () ; (x , y) }) . collect () ; tuples . sort_by (| & (ax , _) , & (bx , _) | ax . partial_cmp (& bx) . unwrap_or (Ordering :: Less)) ; let function_name = key . as_ref () ; let (xs , mut ys) : (Vec < _ > , Vec < _ >) = tuples . into_iter () . unzip () ; formatter . scale_values (max , & mut ys) ; series_data . push ((function_name , xs , ys)) ; } (unit , series_data) }
    };
}

line_comparison_series_data!()