// Generated macro for impl_1371 (impl)
macro_rules! Depcrate_utils_cacheimpl_1371 {
() => {
// Module: crate::utils::cache
// Provides: {"impl_1371"}
// Dependencies: {}
impl Cache { # [doc = " Creates a new empty cache."] pub fn new () -> Cache { Cache :: default () } # [doc = " Stores the result of a computation step in the cache."] pub fn put < S : Step > (& self , step : S , value : S :: Output) { let mut cache = self . cache . borrow_mut () ; let type_id = TypeId :: of :: < S > () ; let stepcache = cache . entry (type_id) . or_insert_with (| | Box :: < HashMap < S , S :: Output > > :: default ()) . downcast_mut :: < HashMap < S , S :: Output > > () . expect ("invalid type mapped") ; assert ! (! stepcache . contains_key (& step) , "processing {step:?} a second time") ; # [cfg (test)] { let metadata = step . metadata () ; self . executed_steps . borrow_mut () . push (ExecutedStep { metadata }) ; } stepcache . insert (step , value) ; } # [doc = " Retrieves a cached result for the given step, if available."] pub fn get < S : Step > (& self , step : & S) -> Option < S :: Output > { let mut cache = self . cache . borrow_mut () ; let type_id = TypeId :: of :: < S > () ; let stepcache = cache . entry (type_id) . or_insert_with (| | Box :: < HashMap < S , S :: Output > > :: default ()) . downcast_mut :: < HashMap < S , S :: Output > > () . expect ("invalid type mapped") ; stepcache . get (step) . cloned () } }
};
}
