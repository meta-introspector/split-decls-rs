macro_rules! deps {
    () => {
        CodegenCx!();
        Visibility!();
        Linkage!();
        SymbolAlreadyDefined!();
        SetUniqueComdat!();
    };
}

macro_rules! impl_573 {
    () => {
        deps!();
        impl < 'tcx > PreDefineCodegenMethods < 'tcx > for CodegenCx < '_ , 'tcx > { fn predefine_static (& mut self , def_id : DefId , linkage : Linkage , visibility : Visibility , symbol_name : & str ,) { let instance = Instance :: mono (self . tcx , def_id) ; let DefKind :: Static { nested , .. } = self . tcx . def_kind (def_id) else { bug ! () } ; let ty = if nested { self . tcx . types . unit } else { instance . ty (self . tcx , self . typing_env ()) } ; let llty = self . layout_of (ty) . llvm_type (self) ; let g = self . define_global (symbol_name , llty) . unwrap_or_else (| | { self . sess () . dcx () . emit_fatal (SymbolAlreadyDefined { span : self . tcx . def_span (def_id) , symbol_name }) }) ; llvm :: set_linkage (g , base :: linkage_to_llvm (linkage)) ; llvm :: set_visibility (g , base :: visibility_to_llvm (visibility)) ; self . assume_dso_local (g , false) ; self . instances . borrow_mut () . insert (instance , g) ; } fn predefine_fn (& mut self , instance : Instance < 'tcx > , linkage : Linkage , visibility : Visibility , symbol_name : & str ,) { assert ! (! instance . args . has_infer ()) ; let fn_abi = self . fn_abi_of_instance (instance , ty :: List :: empty ()) ; let lldecl = self . declare_fn (symbol_name , fn_abi , Some (instance)) ; llvm :: set_linkage (lldecl , base :: linkage_to_llvm (linkage)) ; let attrs = self . tcx . codegen_instance_attrs (instance . def) ; base :: set_link_section (lldecl , & attrs) ; if (linkage == Linkage :: LinkOnceODR || linkage == Linkage :: WeakODR) && self . tcx . sess . target . supports_comdat () { llvm :: SetUniqueComdat (self . llmod , lldecl) ; } if linkage != Linkage :: Internal && self . tcx . is_compiler_builtins (LOCAL_CRATE) { llvm :: set_visibility (lldecl , llvm :: Visibility :: Hidden) ; } else { llvm :: set_visibility (lldecl , base :: visibility_to_llvm (visibility)) ; } debug ! ("predefine_fn: instance = {:?}" , instance) ; self . assume_dso_local (lldecl , false) ; self . instances . borrow_mut () . insert (instance , lldecl) ; } }
    };
}

impl_573!();