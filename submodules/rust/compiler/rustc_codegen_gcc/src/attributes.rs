mkuse!{# [cfg (feature = "master")] use gccjit :: FnAttribute ;}
mkuse!{use gccjit :: Function ;}
mkuse!{# [cfg (feature = "master")] use rustc_hir :: attrs :: InlineAttr ;}
mkuse!{use rustc_hir :: attrs :: InstructionSetAttr ;}
mkuse!{# [cfg (feature = "master")] use rustc_middle :: middle :: codegen_fn_attrs :: CodegenFnAttrFlags ;}
mkuse!{# [cfg (feature = "master")] use rustc_middle :: mir :: TerminatorKind ;}
mkuse!{use rustc_middle :: ty ;}
mkuse!{use crate :: context :: CodegenCx ;}
mkuse!{use crate :: gcc_util :: to_gcc_features ;}

macro_rules! recursively_inline_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function recursively_inline in module {}", module_path!());
    };
}

mkfn!{
    recursively_inline_introspect!();
    # [doc = " Checks if the function `instance` is recursively inline."] # [doc = " Returns `false` if a functions is guaranteed to be non-recursive, and `true` if it *might* be recursive."] # [cfg (feature = "master")] fn recursively_inline < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , instance : ty :: Instance < 'tcx > ,) -> bool { if ! cx . tcx . is_mir_available (instance . def_id ()) { return true ; } let body = cx . tcx . optimized_mir (instance . def_id ()) ; for block in body . basic_blocks . iter () { let Some (ref terminator) = block . terminator else { continue } ; let TerminatorKind :: Call { ref func , .. } = terminator . kind else { continue } ; let Some ((def , _args)) = func . const_fn_def () else { continue } ; if matches ! (cx . tcx . codegen_fn_attrs (def) . inline , InlineAttr :: Always | InlineAttr :: Force { .. }) { return true ; } } false }
}

macro_rules! inline_attr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inline_attr in module {}", module_path!());
    };
}

mkfn!{
    inline_attr_introspect!();
    # [doc = " Get GCC attribute for the provided inline heuristic, attached to `instance`."] # [cfg (feature = "master")] # [inline] fn inline_attr < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , inline : InlineAttr , instance : ty :: Instance < 'tcx > ,) -> Option < FnAttribute < 'gcc > > { match inline { InlineAttr :: Always => { if recursively_inline (cx , instance) { Some (FnAttribute :: Inline) } else { Some (FnAttribute :: AlwaysInline) } } InlineAttr :: Hint => Some (FnAttribute :: Inline) , InlineAttr :: Force { .. } => Some (FnAttribute :: AlwaysInline) , InlineAttr :: Never => { if cx . sess () . target . arch != "amdgpu" { Some (FnAttribute :: NoInline) } else { None } } InlineAttr :: None => None , } }
}

macro_rules! from_fn_attrs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_fn_attrs in module {}", module_path!());
    };
}

mkfn!{
    from_fn_attrs_introspect!();
    # [doc = " Composite function which sets GCC attributes for function depending on its AST (`#[attribute]`)"] # [doc = " attributes."] pub fn from_fn_attrs < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , # [cfg_attr (not (feature = "master") , allow (unused_variables))] func : Function < 'gcc > , instance : ty :: Instance < 'tcx > ,) { let codegen_fn_attrs = cx . tcx . codegen_instance_attrs (instance . def) ; # [cfg (feature = "master")] { let inline = if codegen_fn_attrs . flags . contains (CodegenFnAttrFlags :: NAKED) { InlineAttr :: Never } else if codegen_fn_attrs . inline == InlineAttr :: None && instance . def . requires_inline (cx . tcx) { InlineAttr :: Hint } else { codegen_fn_attrs . inline } ; if let Some (attr) = inline_attr (cx , inline , instance) { if let FnAttribute :: AlwaysInline = attr { func . add_attribute (FnAttribute :: Inline) ; } func . add_attribute (attr) ; } if codegen_fn_attrs . flags . contains (CodegenFnAttrFlags :: COLD) { func . add_attribute (FnAttribute :: Cold) ; } if codegen_fn_attrs . flags . contains (CodegenFnAttrFlags :: FFI_PURE) { func . add_attribute (FnAttribute :: Pure) ; } if codegen_fn_attrs . flags . contains (CodegenFnAttrFlags :: FFI_CONST) { func . add_attribute (FnAttribute :: Const) ; } } let mut function_features = codegen_fn_attrs . target_features . iter () . map (| features | features . name . as_str ()) . flat_map (| feat | to_gcc_features (cx . tcx . sess , feat) . into_iter ()) . chain (codegen_fn_attrs . instruction_set . iter () . map (| x | match * x { InstructionSetAttr :: ArmA32 => "-thumb-mode" , InstructionSetAttr :: ArmT32 => "thumb-mode" , })) . collect :: < Vec < _ > > () ; let mut global_features = cx . tcx . global_backend_features (()) . iter () . map (| s | s . as_str ()) ; function_features . extend (& mut global_features) ; let target_features = function_features . iter () . filter_map (| feature | { if feature . contains ("soft-float") { return None ; } if feature . starts_with ('-') { Some (format ! ("no{}" , feature)) } else if let Some (stripped) = feature . strip_prefix ('+') { Some (stripped . to_string ()) } else { Some (feature . to_string ()) } }) . collect :: < Vec < _ > > () . join (",") ; if ! target_features . is_empty () { # [cfg (feature = "master")] match cx . sess () . target . arch . as_ref () { "x86" | "x86_64" | "powerpc" => { func . add_attribute (FnAttribute :: Target (& target_features)) } _ => () , } } }
}