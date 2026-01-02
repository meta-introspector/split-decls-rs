mkuse!{use std :: fmt ;}
mkuse!{use std :: io :: Write ;}
mkuse!{use cranelift_codegen :: entity :: SecondaryMap ;}
mkuse!{use cranelift_codegen :: ir :: Fact ;}
mkuse!{use cranelift_codegen :: ir :: entities :: AnyEntity ;}
mkuse!{use cranelift_codegen :: write :: { FuncWriter , PlainWriter } ;}
mkuse!{use rustc_middle :: ty :: print :: with_no_trimmed_paths ;}
mkuse!{use rustc_session :: config :: { OutputFilenames , OutputType } ;}
mkuse!{use rustc_target :: callconv :: FnAbi ;}
mkuse!{use crate :: prelude :: * ;}
mkitem!{mkstruct!{# [derive (Clone , Debug)] pub (crate) struct CommentWriter { enabled : bool , global_comments : Vec < String > , entity_comments : FxHashMap < AnyEntity , String > , inst_post_comments : FxHashMap < Inst , String > , }}}
mkitem!{mkimpl!{impl CommentWriter { pub (crate) fn new < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , fn_abi : & 'tcx FnAbi < 'tcx , Ty < 'tcx > > ,) -> Self { let enabled = should_write_ir (tcx) ; let global_comments = if enabled { with_no_trimmed_paths ! ({ vec ! [format ! ("symbol {}" , tcx . symbol_name (instance) . name) , format ! ("instance {:?}" , instance) , format ! ("abi {:?}" , fn_abi) , String :: new () ,] }) } else { vec ! [] } ; CommentWriter { enabled , global_comments , entity_comments : FxHashMap :: default () , inst_post_comments : FxHashMap :: default () , } } }}}
mkitem!{mkimpl!{impl CommentWriter { pub (crate) fn enabled (& self) -> bool { self . enabled } pub (crate) fn add_global_comment < S : Into < String > > (& mut self , comment : S) { debug_assert ! (self . enabled) ; self . global_comments . push (comment . into ()) ; } pub (crate) fn add_comment < S : Into < String > + AsRef < str > , E : Into < AnyEntity > > (& mut self , entity : E , comment : S ,) { debug_assert ! (self . enabled) ; use std :: collections :: hash_map :: Entry ; match self . entity_comments . entry (entity . into ()) { Entry :: Occupied (mut occ) => { occ . get_mut () . push ('\n') ; occ . get_mut () . push_str (comment . as_ref ()) ; } Entry :: Vacant (vac) => { vac . insert (comment . into ()) ; } } } pub (crate) fn add_post_comment < S : Into < String > + AsRef < str > > (& mut self , entity : Inst , comment : S ,) { debug_assert ! (self . enabled) ; use std :: collections :: hash_map :: Entry ; match self . inst_post_comments . entry (entity) { Entry :: Occupied (mut occ) => { occ . get_mut () . push ('\n') ; occ . get_mut () . push_str (comment . as_ref ()) ; } Entry :: Vacant (vac) => { vac . insert (comment . into ()) ; } } } }}}
mkitem!{mkimpl!{impl FuncWriter for & '_ CommentWriter { fn write_preamble (& mut self , w : & mut dyn fmt :: Write , func : & Function ,) -> Result < bool , fmt :: Error > { for comment in & self . global_comments { if ! comment . is_empty () { writeln ! (w , "; {}" , comment) ? ; } else { writeln ! (w) ? ; } } if ! self . global_comments . is_empty () { writeln ! (w) ? ; } self . super_preamble (w , func) } fn write_entity_definition (& mut self , w : & mut dyn fmt :: Write , _func : & Function , entity : AnyEntity , value : & dyn fmt :: Display , maybe_fact : Option < & Fact > ,) -> fmt :: Result { if let Some (fact) = maybe_fact { write ! (w , "    {} ! {} = {}" , entity , fact , value) ? ; } else { write ! (w , "    {} = {}" , entity , value) ? ; } if let Some (comment) = self . entity_comments . get (& entity) { writeln ! (w , " ; {}" , comment . replace ('\n' , "\n; ")) } else { writeln ! (w) } } fn write_block_header (& mut self , w : & mut dyn fmt :: Write , func : & Function , block : Block , indent : usize ,) -> fmt :: Result { PlainWriter . write_block_header (w , func , block , indent) } fn write_instruction (& mut self , w : & mut dyn fmt :: Write , func : & Function , aliases : & SecondaryMap < Value , Vec < Value > > , inst : Inst , indent : usize ,) -> fmt :: Result { if let Some (comment) = self . entity_comments . get (& inst . into ()) { writeln ! (w , "; {}" , comment . replace ('\n' , "\n; ")) ? ; } PlainWriter . write_instruction (w , func , aliases , inst , indent) ? ; if let Some (comment) = self . inst_post_comments . get (& inst) { writeln ! (w , "; {}" , comment . replace ('\n' , "\n; ")) ? ; } Ok (()) } }}}
mkitem!{mkimpl!{impl FunctionCx < '_ , '_ , '_ > { pub (crate) fn add_global_comment < S : Into < String > > (& mut self , comment : S) { self . clif_comments . add_global_comment (comment) ; } pub (crate) fn add_comment < S : Into < String > + AsRef < str > , E : Into < AnyEntity > > (& mut self , entity : E , comment : S ,) { self . clif_comments . add_comment (entity , comment) ; } pub (crate) fn add_post_comment < S : Into < String > + AsRef < str > > (& mut self , entity : Inst , comment : S ,) { self . clif_comments . add_post_comment (entity , comment) ; } }}}

macro_rules! should_write_ir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function should_write_ir in module {}", module_path!());
    };
}

mkfn!{
    should_write_ir_introspect!();
    pub (crate) fn should_write_ir (tcx : TyCtxt < '_ >) -> bool { tcx . sess . opts . output_types . contains_key (& OutputType :: LlvmAssembly) }
}

macro_rules! write_ir_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_ir_file in module {}", module_path!());
    };
}

mkfn!{
    write_ir_file_introspect!();
    pub (crate) fn write_ir_file (output_filenames : & OutputFilenames , name : & str , write : impl FnOnce (& mut dyn Write) -> std :: io :: Result < () > ,) { let clif_output_dir = output_filenames . with_extension ("clif") ; match std :: fs :: create_dir (& clif_output_dir) { Ok (()) => { } Err (err) if err . kind () == std :: io :: ErrorKind :: AlreadyExists => { } res @ Err (_) => res . unwrap () , } let clif_file_name = clif_output_dir . join (name) ; let res = std :: fs :: File :: create (clif_file_name) . and_then (| mut file | write (& mut file)) ; if let Err (err) = res { let handler = rustc_session :: EarlyDiagCtxt :: new (rustc_session :: config :: ErrorOutputType :: default ()) ; handler . early_warn (format ! ("error writing ir file: {}" , err)) ; } }
}

macro_rules! write_clif_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_clif_file in module {}", module_path!());
    };
}

mkfn!{
    write_clif_file_introspect!();
    pub (crate) fn write_clif_file (output_filenames : & OutputFilenames , symbol_name : & str , postfix : & str , isa : & dyn cranelift_codegen :: isa :: TargetIsa , func : & cranelift_codegen :: ir :: Function , mut clif_comments : & CommentWriter ,) { write_ir_file (output_filenames , & format ! ("{}.{}.clif" , symbol_name , postfix) , | file | { let mut clif = String :: new () ; cranelift_codegen :: write :: decorate_function (& mut clif_comments , & mut clif , func) . unwrap () ; for flag in isa . flags () . iter () { writeln ! (file , "set {}" , flag) ? ; } write ! (file , "target {}" , isa . triple () . architecture) ? ; for isa_flag in isa . isa_flags () . iter () { write ! (file , " {}" , isa_flag) ? ; } writeln ! (file , "\n") ? ; writeln ! (file) ? ; file . write_all (clif . as_bytes ()) ? ; Ok (()) }) ; }
}
mkitem!{mkimpl!{impl fmt :: Debug for FunctionCx < '_ , '_ , '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "{:?}" , self . instance . args) ? ; writeln ! (f , "{:?}" , self . local_map) ? ; let mut clif = String :: new () ; :: cranelift_codegen :: write :: decorate_function (& mut & self . clif_comments , & mut clif , & self . bcx . func ,) . unwrap () ; writeln ! (f , "\n{}" , clif) } }}}