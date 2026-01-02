mkuse!{use rustc_ast :: Attribute ;}
mkuse!{use rustc_expand :: base :: resolve_path ;}
mkuse!{use rustc_middle :: middle :: debugger_visualizer :: { DebuggerVisualizerFile , DebuggerVisualizerType } ;}
mkuse!{use rustc_middle :: query :: { LocalCrate , Providers } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use crate :: errors :: { DebugVisualizerInvalid , DebugVisualizerUnreadable } ;}
mkitem!{mkimpl!{impl DebuggerVisualizerCollector < '_ > { fn check_for_debugger_visualizer (& mut self , attr : & Attribute) { if attr . has_name (sym :: debugger_visualizer) { let Some (hints) = attr . meta_item_list () else { self . sess . dcx () . emit_err (DebugVisualizerInvalid { span : attr . span }) ; return ; } ; let [hint] = hints . as_slice () else { self . sess . dcx () . emit_err (DebugVisualizerInvalid { span : attr . span }) ; return ; } ; let Some (meta_item) = hint . meta_item () else { self . sess . dcx () . emit_err (DebugVisualizerInvalid { span : attr . span }) ; return ; } ; let (visualizer_type , visualizer_path) = match (meta_item . name () , meta_item . value_str ()) { (Some (sym :: natvis_file) , Some (value)) => (DebuggerVisualizerType :: Natvis , value) , (Some (sym :: gdb_script_file) , Some (value)) => { (DebuggerVisualizerType :: GdbPrettyPrinter , value) } (_ , _) => { self . sess . dcx () . emit_err (DebugVisualizerInvalid { span : meta_item . span }) ; return ; } } ; let file = match resolve_path (& self . sess , visualizer_path . as_str () , attr . span) { Ok (file) => file , Err (err) => { err . emit () ; return ; } } ; match self . sess . source_map () . load_binary_file (& file) { Ok ((source , _)) => { self . visualizers . push (DebuggerVisualizerFile :: new (source , visualizer_type , file ,)) ; } Err (error) => { self . sess . dcx () . emit_err (DebugVisualizerUnreadable { span : meta_item . span , file : & file , error , }) ; } } } } }}}
mkitem!{mkstruct!{struct DebuggerVisualizerCollector < 'a > { sess : & 'a Session , visualizers : Vec < DebuggerVisualizerFile > , }}}
mkitem!{mkimpl!{impl < 'ast > rustc_ast :: visit :: Visitor < 'ast > for DebuggerVisualizerCollector < '_ > { fn visit_attribute (& mut self , attr : & 'ast Attribute) { self . check_for_debugger_visualizer (attr) ; rustc_ast :: visit :: walk_attribute (self , attr) ; } }}}

macro_rules! debugger_visualizers_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function debugger_visualizers in module {}", module_path!());
    };
}

mkfn!{
    debugger_visualizers_introspect!();
    # [doc = " Traverses and collects the debugger visualizers for a specific crate."] fn debugger_visualizers (tcx : TyCtxt < '_ > , _ : LocalCrate) -> Vec < DebuggerVisualizerFile > { let resolver_and_krate = tcx . resolver_for_lowering () . borrow () ; let krate = & * resolver_and_krate . 1 ; let mut visitor = DebuggerVisualizerCollector { sess : tcx . sess , visualizers : Vec :: new () } ; rustc_ast :: visit :: Visitor :: visit_crate (& mut visitor , krate) ; visitor . visualizers }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { providers . debugger_visualizers = debugger_visualizers ; }
}