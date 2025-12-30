// Generated macro for impl_277 (impl)
macro_rules! Depcrate_json_pathimpl_277 {
() => {
// Module: crate::json::path
// Provides: {"impl_277"}
// Dependencies: {}
impl Path { pub (crate) fn new (raw : & str , segs : Vec < PathSeg >) -> Path { if let Some ((level , name)) = get_local_path_and_level (& segs) { Path :: Local ((level , name , raw . to_owned ())) } else { Path :: Relative ((segs , raw . to_owned ())) } } pub fn parse (raw : & str) -> Result < Path , RenderError > { HandlebarsParser :: parse (Rule :: path , raw) . map (| p | { let parsed = p . flatten () ; let segs = parse_json_path_from_iter (& mut parsed . peekable () , raw . len ()) ; Ok (Path :: new (raw , segs)) }) . map_err (| _ | RenderErrorReason :: InvalidJsonPath (raw . to_owned ())) ? } pub (crate) fn raw (& self) -> & str { match self { Path :: Relative ((_ , ref raw)) => raw , Path :: Local ((_ , _ , ref raw)) => raw , } } pub (crate) fn current () -> Path { Path :: Relative ((Vec :: with_capacity (0) , String :: new ())) } pub (crate) fn with_named_paths (name_segs : & [& str]) -> Path { let segs = name_segs . iter () . map (| n | PathSeg :: Named ((* n) . to_string ())) . collect () ; Path :: Relative ((segs , name_segs . join ("/"))) } pub (crate) fn segs (& self) -> Option < & [PathSeg] > { match self { Path :: Relative ((segs , _)) => Some (segs) , _ => None , } } }
};
}
