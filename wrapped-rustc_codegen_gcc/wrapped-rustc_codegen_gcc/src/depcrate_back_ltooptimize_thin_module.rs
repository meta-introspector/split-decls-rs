// Generated macro for optimize_thin_module (function)
macro_rules! Depcrate_back_ltooptimize_thin_module {
() => {
// Module: crate::back::lto
// Provides: {"optimize_thin_module"}
// Dependencies: {}
pub fn optimize_thin_module (thin_module : ThinModule < GccCodegenBackend > , _cgcx : & CodegenContext < GccCodegenBackend > ,) -> ModuleCodegen < GccContext > { let mut should_combine_object_files = false ; let context = match thin_module . shared . thin_buffers . get (thin_module . idx) { Some (thin_buffer) => Arc :: clone (& thin_buffer . context) , None => { let context = Context :: default () ; let len = thin_module . shared . thin_buffers . len () ; let module = & thin_module . shared . serialized_modules [thin_module . idx - len] ; match * module { SerializedModule :: Local (ref module_buffer) => { let path = module_buffer . 0 . to_str () . expect ("path") ; context . add_driver_option (path) ; should_combine_object_files = true ; } SerializedModule :: FromRlib (_) => unimplemented ! ("from rlib") , SerializedModule :: FromUncompressedFile (_) => { unimplemented ! ("from uncompressed file") } } Arc :: new (SyncContext :: new (context)) } } ; let module = ModuleCodegen :: new_regular (thin_module . name () . to_string () , GccContext { context , should_combine_object_files , relocation_model : RelocModel :: Pic , temp_dir : None , } ,) ; # [allow (clippy :: let_and_return)] module }
};
}
