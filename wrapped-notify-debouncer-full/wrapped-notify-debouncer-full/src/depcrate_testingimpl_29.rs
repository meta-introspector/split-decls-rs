// Generated macro for impl_29 (impl)
macro_rules! Depcrate_testingimpl_29 {
() => {
// Module: crate::testing
// Provides: {"impl_29"}
// Dependencies: {}
impl schema :: State { pub (crate) fn into_debounce_data_inner (self , time : Instant) -> DebounceDataInner < TestCache > { let queues = self . queues . into_iter () . map (| (path , queue) | { let queue = Queue { events : queue . events . into_iter () . map (| event | event . into_debounced_event (time , Some (& path))) . collect :: < VecDeque < _ > > () , } ; (path . into () , queue) }) . collect () ; let cache = self . cache . into_iter () . map (| (path , id) | { let path = PathBuf :: from (path) ; let id = FileId :: new_inode (id , id) ; (path , id) }) . collect :: < HashMap < _ , _ > > () ; let file_system = self . file_system . into_iter () . map (| (path , id) | { let path = PathBuf :: from (path) ; let id = FileId :: new_inode (id , id) ; (path , id) }) . collect :: < HashMap < _ , _ > > () ; let cache = TestCache :: new (cache , file_system) ; let rename_event = self . rename_event . map (| e | { let file_id = e . file_id . map (| id | FileId :: new_inode (id , id)) ; let event = e . into_debounced_event (time , None) ; (event , file_id) }) ; let rescan_event = self . rescan_event . map (| e | e . into_debounced_event (time , None)) ; DebounceDataInner { queues , roots : Vec :: new () , cache , rename_event , rescan_event , errors : Vec :: new () , timeout : Duration :: from_millis (self . timeout . unwrap_or (50)) , } } }
};
}
