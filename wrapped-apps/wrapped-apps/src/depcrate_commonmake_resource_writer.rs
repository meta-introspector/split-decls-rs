// Generated macro for make_resource_writer (function)
macro_rules! Depcrate_commonmake_resource_writer {
() => {
// Module: crate::common
// Provides: {"make_resource_writer"}
// Dependencies: {}
# [doc = " Makes a buffered writer for a resource with a target URL."] # [doc = ""] # [doc = " The file will have the same name as the resource's last path segment value."] # [doc = " Multiple requests for the same URL are indicated by the value of `cardinal`,"] # [doc = " any value \"N\" greater than 1, will cause \".N\" to be appended to the"] # [doc = " filename."] fn make_resource_writer (url : & url :: Url , target_path : & Option < String > , cardinal : u64 ,) -> Option < std :: io :: BufWriter < std :: fs :: File > > { if let Some (tp) = target_path { let resource = url . path_segments () . map (| c | c . collect :: < Vec < _ > > ()) . unwrap () ; let mut path = format ! ("{}/{}" , tp , resource . iter () . last () . unwrap ()) ; if cardinal > 1 { path = format ! ("{path}.{cardinal}") ; } match std :: fs :: File :: create (& path) { Ok (f) => return Some (std :: io :: BufWriter :: new (f)) , Err (e) => panic ! ("Error creating file for {url}, attempted path was {path}: {e}") , } } None }
};
}
