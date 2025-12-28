macro_rules! deps {
    () => {
        Join!();
    };
}

macro_rules! try_join {
    () => {
        deps!();
        # [doc = " The `try_join!` macro."] pub (crate) fn try_join (input : TokenStream) -> TokenStream { let parsed = syn :: parse_macro_input ! (input as Join) ; let span = Span :: call_site () ; let (future_let_bindings , future_names) = bind_futures (parsed . fut_exprs , span) ; let poll_futures = future_names . iter () . map (| fut | { quote ! { if __futures_crate :: future :: Future :: poll (# fut . as_mut () , __cx) . is_pending () { __all_done = false ; } else if # fut . as_mut () . output_mut () . unwrap () . is_err () { # [allow (unreachable_code)] return __futures_crate :: task :: Poll :: Ready (__futures_crate :: Err (# fut . as_mut () . take_output () . unwrap () . err () . unwrap ())) ; } } }) ; let take_outputs = future_names . iter () . map (| fut | { quote ! { # [allow (unreachable_code)] # fut . as_mut () . take_output () . unwrap () . ok () . unwrap () , } }) ; TokenStream :: from (quote ! { { # (# future_let_bindings) * # [allow (clippy :: diverging_sub_expression)] __futures_crate :: future :: poll_fn (move | __cx : & mut __futures_crate :: task :: Context <'_ >| { let mut __all_done = true ; # (# poll_futures) * if __all_done { __futures_crate :: task :: Poll :: Ready (__futures_crate :: Ok ((# (# take_outputs) *))) } else { __futures_crate :: task :: Poll :: Pending } }) . await } }) }
    };
}

try_join!();