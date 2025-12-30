// Generated macro for impl_494 (impl)
macro_rules! Depcrate_objects_jobject_arrayimpl_494 {
() => {
// Module: crate::objects::jobject_array
// Provides: {"impl_494"}
// Dependencies: {}
impl < E : Reference + Send + Sync > JObjectArrayAPI < E > { fn get < 'any_local > (env : & Env < '_ > , loader_context : & LoaderContext < 'any_local , '_ > ,) -> Result < & 'static Self > { let map = API_REGISTRY . get_or_init (| | RwLock :: new (HashMap :: new ())) ; let tid = TypeId :: of :: < Self > () ; if let Some (any_ref) = map . read () . unwrap () . get (& tid) { return Ok (any_ref . downcast_ref :: < Self > () . expect ("TypeId matched but downcast failed")) ; } let created : JObjectArrayAPI < E > = { env . with_local_frame (DEFAULT_LOCAL_FRAME_CAPACITY , | env | -> Result < _ > { let class = loader_context . load_class_for_type :: < JObjectArray < E > > (false , env) ? ; let class = env . new_global_ref (& class) ? ; Ok (JObjectArrayAPI { class , _marker : std :: marker :: PhantomData , }) }) ? } ; let mut write = map . write () . unwrap () ; if let Some (any_ref) = write . get (& tid) { let api = any_ref . downcast_ref :: < Self > () . expect ("TypeId matched but downcast failed") ; return Ok (api) ; } let leaked : & 'static JObjectArrayAPI < E > = Box :: leak (Box :: new (created)) ; write . insert (tid , leaked as & 'static (dyn Any + Send + Sync)) ; Ok (leaked) } }
};
}
