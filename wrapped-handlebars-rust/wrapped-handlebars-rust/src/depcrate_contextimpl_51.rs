// Generated macro for impl_51 (impl)
macro_rules! Depcrate_contextimpl_51 {
() => {
// Module: crate::context
// Provides: {"impl_51"}
// Dependencies: {}
impl Context { # [doc = " Create a context with null data"] pub fn null () -> Context { Context { data : Json :: Null } } # [doc = " Create a context with given data"] pub fn wraps < T : Serialize > (e : T) -> Result < Context , RenderError > { to_value (e) . map_err (| e | RenderErrorReason :: SerdeError (e) . into ()) . map (| d | Context { data : d }) } # [doc = " Navigate the context with relative path and block scopes"] pub (crate) fn navigate < 'rc > (& 'rc self , relative_path : & [PathSeg] , block_contexts : & VecDeque < BlockContext < '_ > > ,) -> Result < ScopedJson < 'rc > , RenderError > { let resolved_visitor = parse_json_visitor (relative_path , block_contexts , true) ; match resolved_visitor { ResolvedPath :: AbsolutePath (paths) => { let mut ptr = Some (self . data ()) ; for p in & paths { ptr = get_data (ptr , p) ? ; } Ok (ptr . map_or_else (| | ScopedJson :: Missing , | v | ScopedJson :: Context (v , paths))) } ResolvedPath :: RelativePath (_paths) => { unreachable ! () } ResolvedPath :: BlockParamValue (paths , value) | ResolvedPath :: LocalValue (paths , value) => { let mut ptr = Some (value) ; for p in & paths { ptr = get_data (ptr , p) ? ; } Ok (ptr . map_or_else (| | ScopedJson :: Missing , | v | ScopedJson :: Derived (v . clone ()))) } } } # [doc = " Return the Json data wrapped in context"] pub fn data (& self) -> & Json { & self . data } # [doc = " Return the mutable reference to Json data wrapped in context"] pub fn data_mut (& mut self) -> & mut Json { & mut self . data } }
};
}
