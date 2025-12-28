macro_rules! deps {
    () => {
        FormatRedundantArgsSugg!();
        FormatRedundantArgs!();
    };
}

macro_rules! report_redundant_format_arguments {
    () => {
        deps!();
        # [doc = " This function detects and reports unused format!() arguments that are"] # [doc = " redundant due to implicit captures (e.g. `format!(\"{x}\", x)`)."] fn report_redundant_format_arguments < 'a > (ecx : & ExtCtxt < 'a > , args : & FormatArguments , used : & [bool] , placeholders : Vec < (Span , & str) > ,) -> Option < Diag < 'a > > { let mut fmt_arg_indices = vec ! [] ; let mut args_spans = vec ! [] ; let mut fmt_spans = vec ! [] ; for (i , unnamed_arg) in args . unnamed_args () . iter () . enumerate () . rev () { let Some (ty) = unnamed_arg . expr . to_ty () else { continue } ; let Some (argument_binding) = ty . kind . is_simple_path () else { continue } ; let argument_binding = argument_binding . as_str () ; if used [i] { continue ; } let matching_placeholders = placeholders . iter () . filter (| (_ , inline_binding) | argument_binding == * inline_binding) . map (| (span , _) | span) . collect :: < Vec < _ > > () ; if ! matching_placeholders . is_empty () { fmt_arg_indices . push (i) ; args_spans . push (unnamed_arg . expr . span) ; for span in & matching_placeholders { if fmt_spans . contains (* span) { continue ; } fmt_spans . push (* * span) ; } } } if ! args_spans . is_empty () { let multispan = MultiSpan :: from (fmt_spans) ; let mut suggestion_spans = vec ! [] ; for (arg_span , fmt_arg_idx) in args_spans . iter () . zip (fmt_arg_indices . iter ()) { let span = if fmt_arg_idx + 1 == args . explicit_args () . len () { * arg_span } else { arg_span . until (args . explicit_args () [* fmt_arg_idx + 1] . expr . span) } ; suggestion_spans . push (span) ; } let sugg = if args . named_args () . len () == 0 { Some (errors :: FormatRedundantArgsSugg { spans : suggestion_spans }) } else { None } ; return Some (ecx . dcx () . create_err (errors :: FormatRedundantArgs { n : args_spans . len () , span : MultiSpan :: from (args_spans) , note : multispan , sugg , })) ; } None }
    };
}

report_redundant_format_arguments!();