macro_rules! deps {
    () => {
        CreateAttrStringValue!();
        SBuilder!();
        Visibility!();
        SimpleCx!();
        AttributePlace!();
        AttributeKind!();
        UnnamedAddr!();
        CallConv!();
        SmallVec!();
    };
}

macro_rules! create_wrapper_function {
    () => {
        deps!();
        fn create_wrapper_function (tcx : TyCtxt < '_ > , cx : & SimpleCx < '_ > , from_name : & str , to_name : Option < & str > , args : & [& Type] , output : Option < & Type > , no_return : bool ,) { let ty = cx . type_func (args , output . unwrap_or_else (| | cx . type_void ())) ; let llfn = declare_simple_fn (& cx , from_name , llvm :: CallConv :: CCallConv , llvm :: UnnamedAddr :: Global , llvm :: Visibility :: from_generic (tcx . sess . default_visibility ()) , ty ,) ; let mut attrs = SmallVec :: < [_ ; 2] > :: new () ; let target_cpu = llvm_util :: target_cpu (tcx . sess) ; let target_cpu_attr = llvm :: CreateAttrStringValue (cx . llcx , "target-cpu" , target_cpu) ; let tune_cpu_attr = llvm_util :: tune_cpu (tcx . sess) . map (| tune_cpu | llvm :: CreateAttrStringValue (cx . llcx , "tune-cpu" , tune_cpu)) ; attrs . push (target_cpu_attr) ; attrs . extend (tune_cpu_attr) ; attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Function , & attrs) ; let no_return = if no_return { let no_return = llvm :: AttributeKind :: NoReturn . create_attr (cx . llcx) ; attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Function , & [no_return]) ; Some (no_return) } else { None } ; if tcx . sess . must_emit_unwind_tables () { let uwtable = attributes :: uwtable_attr (cx . llcx , tcx . sess . opts . unstable_opts . use_sync_unwind) ; attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Function , & [uwtable]) ; } let llbb = unsafe { llvm :: LLVMAppendBasicBlockInContext (cx . llcx , llfn , c"entry" . as_ptr ()) } ; let mut bx = SBuilder :: build (& cx , llbb) ; if let Some (to_name) = to_name { let callee = declare_simple_fn (& cx , to_name , llvm :: CallConv :: CCallConv , llvm :: UnnamedAddr :: Global , llvm :: Visibility :: Hidden , ty ,) ; if let Some (no_return) = no_return { attributes :: apply_to_llfn (callee , llvm :: AttributePlace :: Function , & [no_return]) ; } llvm :: set_visibility (callee , llvm :: Visibility :: Hidden) ; let args = args . iter () . enumerate () . map (| (i , _) | llvm :: get_param (llfn , i as c_uint)) . collect :: < Vec < _ > > () ; let ret = bx . call (ty , callee , & args , None) ; llvm :: LLVMSetTailCall (ret , TRUE) ; if output . is_some () { bx . ret (ret) ; } else { bx . ret_void () } } else { assert ! (output . is_none ()) ; bx . ret_void () } }
    };
}

create_wrapper_function!()