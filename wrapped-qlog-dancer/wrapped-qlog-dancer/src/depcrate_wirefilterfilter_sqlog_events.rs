// Generated macro for filter_sqlog_events (function)
macro_rules! Depcrate_wirefilterfilter_sqlog_events {
() => {
// Module: crate::wirefilter
// Provides: {"filter_sqlog_events"}
// Dependencies: {}
pub fn filter_sqlog_events (mut events : Vec < Event > , filter : & str) -> Vec < Event > { let mut ret = vec ! [] ; let mut builder = Scheme ! { category : Bytes , name : Bytes , stream_id : Array (Int) , } ; builder . add_function ("any" , wirefilter :: AnyFunction { }) . unwrap () ; let scheme = builder . build () ; let ast = scheme . parse (filter) . unwrap () ; let filter = ast . compile () ; for event in events . drain (..) { let mut ctx = ExecutionContext :: new (& scheme) ; let filter_match = match & event { Event :: Qlog (ev) => { let (cat , ty) = category_and_type_from_event (& ev) ; ctx . set_field_value (scheme . get_field ("category") . unwrap () , cat . clone () ,) . unwrap () ; ctx . set_field_value (scheme . get_field ("name") . unwrap () , ty . clone () ,) . unwrap () ; ctx . set_field_value (scheme . get_field ("stream_id") . unwrap () , stream_ids (& event) ,) . unwrap () ; filter . execute (& ctx) . unwrap () } , Event :: Json (ev) => { let (cat , ty) = category_and_type_from_event (& ev) ; ctx . set_field_value (scheme . get_field ("category") . unwrap () , cat . clone () ,) . unwrap () ; ctx . set_field_value (scheme . get_field ("name") . unwrap () , ty . clone () ,) . unwrap () ; ctx . set_field_value (scheme . get_field ("stream_id") . unwrap () , stream_ids (& event) ,) . unwrap () ; filter . execute (& ctx) . unwrap () } , } ; if filter_match { ret . push (event) ; } } ret }
};
}
