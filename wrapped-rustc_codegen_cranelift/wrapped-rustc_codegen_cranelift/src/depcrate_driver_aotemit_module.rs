// Generated macro for emit_module (function)
macro_rules! Depcrate_driver_aotemit_module {
() => {
// Module: crate::driver::aot
// Provides: {"emit_module"}
// Dependencies: {}
fn emit_module (output_filenames : & OutputFilenames , invocation_temp : Option < & str > , prof : & SelfProfilerRef , mut object : cranelift_object :: object :: write :: Object < '_ > , kind : ModuleKind , name : String , producer_str : & str ,) -> Result < CompiledModule , String > { if object . format () == cranelift_object :: object :: BinaryFormat :: Elf { let comment_section = object . add_section (Vec :: new () , b".comment" . to_vec () , cranelift_object :: object :: SectionKind :: OtherString ,) ; let mut producer = vec ! [0] ; producer . extend (producer_str . as_bytes ()) ; producer . push (0) ; object . set_section_data (comment_section , producer , 1) ; } let tmp_file = output_filenames . temp_path_for_cgu (OutputType :: Object , & name , invocation_temp) ; let file = match File :: create (& tmp_file) { Ok (file) => file , Err (err) => return Err (format ! ("error creating object file: {}" , err)) , } ; let mut file = BufWriter :: new (file) ; if let Err (err) = object . write_stream (& mut file) { return Err (format ! ("error writing object file: {}" , err)) ; } let file = match file . into_inner () { Ok (file) => file , Err (err) => return Err (format ! ("error writing object file: {}" , err)) , } ; if prof . enabled () { prof . artifact_size ("object_file" , tmp_file . file_name () . unwrap () . to_string_lossy () , file . metadata () . unwrap () . len () ,) ; } Ok (CompiledModule { name , kind , object : Some (tmp_file) , dwarf_object : None , bytecode : None , assembly : None , llvm_ir : None , links_from_incr_cache : Vec :: new () , }) }
};
}
