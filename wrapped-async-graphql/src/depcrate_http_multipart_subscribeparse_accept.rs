// Generated macro for parse_accept (function)
macro_rules! Depcrate_http_multipart_subscribeparse_accept {
() => {
// Module: crate::http::multipart_subscribe
// Provides: {"parse_accept"}
// Dependencies: {}
fn parse_accept (accept : & str) -> Vec < Mime > { let mut items = accept . split (',') . map (str :: trim) . filter_map (| item | { let mime : Mime = item . parse () . ok () ? ; let q = mime . get_param ("q") . and_then (| value | Some ((value . as_str () . parse :: < f32 > () . ok () ? * 1000.0) as i32)) . unwrap_or (1000) ; Some ((mime , q)) }) . collect :: < Vec < _ > > () ; items . sort_by (| (_ , qa) , (_ , qb) | qb . cmp (qa)) ; items . into_iter () . map (| (mime , _) | mime) . collect () }
};
}
