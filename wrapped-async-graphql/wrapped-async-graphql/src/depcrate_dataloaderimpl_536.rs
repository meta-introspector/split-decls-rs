// Generated macro for impl_536 (impl)
macro_rules! Depcrate_dataloaderimpl_536 {
() => {
// Module: crate::dataloader
// Provides: {"impl_536"}
// Dependencies: {}
impl < T > DataLoaderInner < T > { # [cfg_attr (feature = "tracing" , instrument (skip_all))] async fn do_load < K > (& self , disable_cache : bool , (keys , senders) : KeysAndSender < K , T >) where K : Send + Sync + Hash + Eq + Clone + 'static , T : Loader < K > , { let tid = TypeId :: of :: < K > () ; let keys = keys . into_iter () . collect :: < Vec < _ > > () ; match self . loader . load (& keys) . await { Ok (values) => { let mut request = self . requests . lock () . unwrap () ; let typed_requests = request . get_mut (& tid) . unwrap () . downcast_mut :: < Requests < K , T > > () . unwrap () ; let disable_cache = typed_requests . disable_cache || disable_cache ; if ! disable_cache { for (key , value) in & values { typed_requests . cache_storage . insert (Cow :: Borrowed (key) , Cow :: Borrowed (value)) ; } } for (keys , sender) in senders { let mut res = HashMap :: new () ; res . extend (sender . use_cache_values) ; for key in & keys { res . extend (values . get (key) . map (| value | (key . clone () , value . clone ()))) ; } sender . tx . send (Ok (res)) . ok () ; } } Err (err) => { for (_ , sender) in senders { sender . tx . send (Err (err . clone ())) . ok () ; } } } } }
};
}
